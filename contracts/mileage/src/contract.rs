#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point,
};
use log::info;
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{DetailsResponse, ExecuteMsg, InstantiateMsg, InsurancePolicyData, MotusByAddressResponse, PaymentVerificationResponse, QueryMsg, WithdrawMsg};
use crate::policy::Policy;
use crate::state::{State, POLICES, STATE};
use crate::querier::SoarchainQuerier;
use crate::query::SoarchainQuery;
use cosmwasm_std::{
    coin, to_json_binary, BalanceResponse, BankMsg, BankQuery, Binary, StdResult
};
use crate::utils::calculate_mileage;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:mileage-contract";
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
        policy_holder: msg.policy_holder.to_string(),
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
        .add_attribute("owner", info.sender)
        .add_attribute("motus_owner", msg.insured_party))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePolicy(msg) => create_mileage_based_policy(deps, env, msg, info ),
        ExecuteMsg::Withdraw(msg)  => execute_withdraw(deps, env, info, msg),
        ExecuteMsg::Close{insured_party} => execute_close(deps, env, insured_party, info),
    }
}

pub fn create_mileage_based_policy(
    deps: DepsMut,
    env: Env,
    msg: InsurancePolicyData,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
  
    let policy_holder = deps.api.addr_validate(&msg.policy.policy_holder)?;
    let insured_party = deps.api.addr_validate(&msg.policy.insured_party)?;

    // Ensure that the sender is the owner of the contract
    let state = STATE.load(deps.storage)?;
    if state.policy_holder != msg.policy.policy_holder {
        return Err(ContractError::Unauthorized {});
    }

    if msg.data.len() < 2 {
        return Err(ContractError::NoData {});
    }

    let mile = calculate_mileage(&msg.data);

    // Calculate the insurance premium based on miles driven
    let premium = state.base_rate + (mile * state.rate_per_mile);

    let policy = Policy::create(
        msg.policy.id.to_string(),
        policy_holder.to_string(),
        insured_party.to_string(),
        env.block.time.seconds().into(),
        msg.policy.beneficiary,
        msg.policy.coverage,
        msg.policy.plan,
        premium,
        msg.policy.period,
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
    _env: Env,
    _info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;

    let policy = POLICES.load(deps.storage, &msg.policy_holder)?;
    let withdraw_amount: u128 = policy.premium as u128;

    // Ensure the sender has enough funds to transfer
    if withdraw_amount.eq(&0) {
        return Result::Err(ContractError::ZeroAmount {});
    }

    if policy.closed {
        return Err(ContractError::Closed {});
    }
    POLICES.save(deps.storage, &msg.policy_holder, &policy)?;

    let msg = BankMsg::Send {
        to_address: policy.policy_holder.to_string(),
        amount: vec![coin(withdraw_amount, state.denom.to_string())],
    };

    let res = Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("to", policy.policy_holder)
        .add_attribute("amount", withdraw_amount.to_string())
        .add_message(msg);
    Ok(res)
}

pub fn execute_close(
    deps: DepsMut,
    _env: Env,
    motus_owner: String,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let mut policy = POLICES.load(deps.storage, &motus_owner)?;

    if policy.closed {
        return Err(ContractError::Closed {  });
    }

    // Ensure that the sender is the owner of the contract
    if info.sender != policy.policy_holder {
        return Err(ContractError::InvalidUser { });
    }

    policy.closed = true;
    
    POLICES.save(deps.storage, &motus_owner, &policy)?;

    let withdraw_amount: u128 = policy.premium as u128;
    let state = STATE.load(deps.storage)?;

    let msg = BankMsg::Send {
        to_address: policy.insured_party.to_string(),
        amount: vec![coin(withdraw_amount, state.denom.to_string())],
    };

    let res = Response::new()
        .add_attribute("action", "close")
        .add_message(msg);
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
        creation_date: policy.creation_date,
        beneficiary: policy.beneficiary,
        coverage: policy.coverage,
        plan: policy.plan,
        premium: policy.premium,
        period: policy.period,
        closed: policy.closed
    };

    Ok(details)
}