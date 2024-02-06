#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point,
};
use log::info;
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{DetailsResponse, ExecuteMsg, InstantiateMsg, InsurancePolicyData, MotusByAddressResponse, PaymentVerificationResponse, QueryMsg, RenewalMsg, TerminateMsg, WithdrawMsg};
use crate::policy::Policy;
use crate::state::{State, POLICES, STATE};
use crate::querier::SoarchainQuerier;
use crate::query::SoarchainQuery;
use cosmwasm_std::{
    coin, to_json_binary, BalanceResponse, BankMsg, BankQuery, Binary, StdResult
};
use crate::utils::{calculate_avg_rpm, calculate_avg_vss, calculate_renewal_termination_date};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:usage-based-insurance-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<SoarchainQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Create the initial state for the contract
    let state = State {
        policy_holder: info.sender.to_string(),
        insured_party: msg.insured_party.to_string(),
        denom: msg.denom,
        base_rate: msg.base_rate,
        rate_per_mile: msg.rate_per_mileage,
    };


    // Save the contract version and initial state to storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    // Return a success response with relevant attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("policy_holder", info.sender)
        .add_attribute("insured_party", msg.insured_party))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePolicy(msg) => create_usage_based_policy(deps, env, msg, info ),
        ExecuteMsg::Withdraw(msg)  => execute_withdraw(deps, env, info, msg),
        ExecuteMsg::Renewal(msg) => execute_renewal(deps, env, info, msg),
        ExecuteMsg::Terminate(msg) => execute_terminate(deps, env, info, msg),
    }
}

pub fn create_usage_based_policy(
    deps: DepsMut,
    env: Env,
    msg: InsurancePolicyData,
    info: MessageInfo,
) -> Result<Response, ContractError> {
  
    let policy_holder = deps.api.addr_validate(&msg.policy.policy_holder)?;
    let insured_party = deps.api.addr_validate(&msg.policy.insured_party)?;

    // Ensure that the sender is the owner of the contract
    if info.sender.to_string() != msg.policy.policy_holder.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    if msg.data.len() < 2 {
        return Err(ContractError::NoData {});
    }

    let mut premium = 0;

    let avg_vss = calculate_avg_vss(&msg.data);
    let avg_rpm = calculate_avg_rpm(&msg.data);

    if avg_vss < 80 &&  avg_rpm < 2500 {
        premium = premium / 2
    } 

    let policy = Policy::create(
        msg.policy.id.to_string(),
        policy_holder.to_string(),
        insured_party.to_string(),
        env.block.time.seconds().into(),
        msg.policy.beneficiary,
        msg.policy.coverage,
        msg.policy.plan,
        premium,
        msg.policy.duration,
        msg.policy.termination_date,
        false,
        false,
    )?;

    POLICES.update(deps.storage, &msg.policy.id, |existing| match existing {
        None => Ok(policy),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("id", msg.policy.id.as_str())]);
    Ok(res)
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;

    let policy = POLICES.load(deps.storage, &msg.id)?;
    let withdraw_amount: u128 = policy.premium as u128;

    let balance = deps.querier.query_balance(env.contract.address.to_string(), state.denom.to_string())?;

    // Ensure the sender has enough funds to transfer
    if withdraw_amount.eq(&0) || balance.amount.u128() < withdraw_amount {
        return Result::Err(ContractError::ZeroAmount {});
    }

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    POLICES.update(deps.storage, &msg.id, |existing| {
        if let Some(mut res) = existing {
            // Modify the existing policy fields as needed
            res.is_active = true;
            // Add more fields if needed
    
            Ok(res)
        } else {
            Err(ContractError::PolicyNotFound {})
        }
    })?;

    let msg = BankMsg::Send {
        to_address: policy.policy_holder.to_string(),
        amount: vec![coin(withdraw_amount, state.denom.to_string())],
    };

    let res = Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("to", policy.policy_holder)
        .add_attribute("withdraw_amount", withdraw_amount.to_string())
        .add_message(msg);
    Ok(res)
}

pub fn execute_renewal(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: RenewalMsg,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;

    // Ensure that the sender is the insured of the contract
    if state.insured_party.to_string() != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
    }

    let policy = POLICES.load(deps.storage, &msg.id)?;
    let renewal_premium: u128 = msg.premium as u128;

    let balance = deps.querier.query_balance(env.contract.address.to_string(), state.denom.to_string())?;

    // Ensure the sender has enough funds to transfer
    if renewal_premium.eq(&0) || balance.amount.u128() < renewal_premium {
        return Result::Err(ContractError::ZeroAmount {});
    }

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    if !policy.is_active {
        return Err(ContractError::NoActive {});
    }

    let termination_time = calculate_renewal_termination_date(policy.duration, policy.termination_date);

    POLICES.update(deps.storage, &msg.id, |existing| {
        if let Some(mut res) = existing {
            // Modify the existing policy fields as needed
            //res.coverage = msg.coverage;
            res.premium= msg.premium;
            res.duration = msg.duration;
            res.termination_date= termination_time;
            // Add more fields if needed
    
            Ok(res)
        } else {
            Err(ContractError::PolicyNotFound {})
        }
    })?;

    let bank_msg = BankMsg::Send {
        to_address: policy.policy_holder.to_string(),
        amount: vec![coin(renewal_premium, state.denom.to_string())],
    };

    let res = Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("to", state.policy_holder)
        .add_attribute("renewal_premium", renewal_premium.to_string())
        .add_message(bank_msg);
    Ok(res)
}

pub fn execute_terminate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: TerminateMsg,
) -> Result<Response, ContractError> {

    // let state = STATE.load(deps.storage)?;
    let policy = POLICES.load(deps.storage, &msg.id)?;

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    // Ensure that the sender is the owner of the contract
    if info.sender.to_string() != policy.policy_holder.to_string() {
        return Err(ContractError::InvalidUser {});
    }

    if policy.is_active {
        return Err(ContractError::Active {});
    }

    POLICES.update(deps.storage, &msg.id, |existing| {
        if let Some(mut res) = existing {
            // Modify the existing policy fields as needed
            res.is_active = false;
            res.closed= true;
            // Add more fields if needed
    
            Ok(res)
        } else {
            Err(ContractError::PolicyNotFound {})
        }
    })?;
    

    let res = Response::new()
        .add_attribute("action", "terminate");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<SoarchainQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {

    match msg { 
        QueryMsg::MotusByAddress { address, } => to_json_binary(&get_motus_by_address(deps, address)), 
        QueryMsg::PaymentVerification {} => to_json_binary(&verify_payment(deps, env)?),
        QueryMsg::Details { id } => to_json_binary(&query_details(deps, id)?),
    }
}

fn get_motus_by_address(deps: Deps<SoarchainQuery>, index: String) -> MotusByAddressResponse {
    info!("contract-insurance-get_motus_by_address: address {}  Mb/s", index);
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(index).unwrap();
    info!("contract-insurance-get_motus_by_address: response {}  Mb/s", response.address);

    MotusByAddressResponse {
        address: response.address,
        pubkey: response.pubkey,
        vin: response.vin,
    }
}

fn verify_payment(deps: Deps<SoarchainQuery>, env: Env) -> StdResult<PaymentVerificationResponse> {

    let state = STATE.load(deps.storage)?;

    let denom = state.denom;
    let contract_address = env.contract.address.to_string();

    let balance_query = BankQuery::Balance { address: contract_address, denom };
    let balance_response: BalanceResponse = deps.querier.query(&balance_query.into())?;
    let balance_u128 = balance_response.amount.amount.u128();
    if balance_u128 > 1 {
        Ok(PaymentVerificationResponse { verified: true })
    } else {
        Ok(PaymentVerificationResponse { verified: false })
    }
}

fn query_details(deps: Deps<SoarchainQuery>, id: String) -> StdResult<DetailsResponse> {
    let policy = POLICES.load(deps.storage, &id)?;

    let details = DetailsResponse {
        id,
        policy_holder: policy.policy_holder,
        insured_party: policy.insured_party,
        start_date: policy.start_date,
        beneficiary: policy.beneficiary,
        coverage: policy.coverage,
        plan: policy.plan,
        premium: policy.premium,
        duration: policy.duration,
        termination_date: policy.termination_date,
        is_active: policy.is_active,
        closed: policy.closed
    };

    Ok(details)
}