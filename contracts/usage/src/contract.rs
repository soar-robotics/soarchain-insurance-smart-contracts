use std::ops::Mul;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point,
};
use log::info;
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{CreateMsg, DetailsResponse, ExecuteMsg, InstantiateMsg, ListResponse, MotusByAddressResponse, PaymentVerificationResponse, QueryMsg, RenewalMsg, TerminateMsg, WithdrawMsg};
use crate::policy::Policy;
use crate::state::{all_policy_insured_parties, State, POLICES, STATE};
use crate::querier::SoarchainQuerier;
use crate::query::SoarchainQuery;
use cosmwasm_std::{
    coin, to_json_binary, BankMsg, Binary, StdResult
};
use crate::utils::{calculate_premium, calculate_renewal_termination_time, calculate_termination_time, create_policy_id, is_policy_eligible_for_renewal};

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
        insurer: info.sender.to_string(),
        denom: msg.denom,
    };

    // Save the contract version and initial state to storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    // Return a success response with relevant attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("insurer", msg.insurer))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateUsageBasedPolicy(msg) => create_policy(deps, env, msg, info ),
        ExecuteMsg::Withdraw(msg)  => execute_withdraw(deps, env, info, msg),
        ExecuteMsg::Renewal(msg) => execute_renewal(deps, env, info, msg),
        ExecuteMsg::Terminate(msg) => execute_terminate(deps, env, info, msg),
    }
}

pub fn create_policy(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    msg: CreateMsg,
    info: MessageInfo,
) -> Result<Response, ContractError> {
  
    // Ensure that the sender is the owner of the contract
    let insurer = deps.api.addr_validate(&msg.insurer.to_string())?;
    let sender = deps.api.addr_validate(&info.sender.to_string())?;

    if insurer != sender {
        return Err(ContractError::Unauthorized {});
    }

    // Verify that the insured party is registered as a motus client within the blockchain.
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(msg.insured_party.to_string(), msg.dpr.to_string()).unwrap();
    if response.address.to_string() != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
    }

    if response.pubkey.to_string() ==  "".to_string() {
        return Err(ContractError::NoDprForInsuredParty {});
    }

    if msg.vehicle_data.len() < 2 {
        return Err(ContractError::NoData {});
    }

    /* TODO: <<Insert your specific business logic calculations in this section.>> */
    
    /* 
     * Guidence:
     * Premium = Avg_Vss & Avg_Rpt & mileage
     * The formula provided serves as a basic illustration of the relationship between rate, mileage, and premium.
     * Insurance companies may use more complex formulas, taking into account various factors such
     * as the type of coverage, the insured vehicle's characteristics, the driver's history OR using the range for premium.
     * For example for Avg_Vss < 80 includes 50% discount
     */

    let policy_id = create_policy_id(&msg.insurer, &msg.insured_party, env.block.time.seconds());

    let termination_time = calculate_termination_time(env.block.time.seconds(), msg.duration);

    // let avg_vss = calculate_avg_vss(&msg.vehicle_data);
    // let avg_rpm = calculate_avg_rpm(&msg.vehicle_data);
    // let mut premium = 0;

    // if avg_vss < 80 &&  avg_rpm < 2500 {
    //     premium = BASE_RATE / 2
    // } else {
    //     premium = BASE_RATE.mul(2);
    // }

    let policy = Policy::create(
        policy_id.to_string(),
        msg.insurer.to_string(),
        response.address.to_string(),
        env.block.time.seconds(),
        calculate_premium(&msg.vehicle_data),
        msg.duration,
        termination_time,
        false,
        false,
    )?;

    POLICES.update(deps.storage, &msg.insured_party, |existing| match existing {
        None => Ok(policy),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("insured_party", &msg.insured_party)]);
    Ok(res)
}

pub fn execute_withdraw(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    _info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;
    let policy = POLICES.load(deps.storage, &msg.insured_party.to_string())?;

    if msg.insured_party.to_string() != policy.insured_party.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure that the insured oarty is the motus owner
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(msg.insured_party.to_string(), msg.dpr.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty{});
    }

    if response.pubkey.to_string() ==  "".to_string() {
        return Err(ContractError::NoDprForInsuredParty {});
    }

    let withdraw_amount: u128 = policy.premium as u128;
    let balance = deps.querier.query_balance(env.contract.address.to_string(), state.denom.to_string())?;

    // Ensure the sender has enough funds to transfer
    if withdraw_amount.eq(&0) || balance.amount.u128() < withdraw_amount {
        return Result::Err(ContractError::ZeroAmount {});
    }

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    POLICES.update(deps.storage, &msg.insured_party.to_string(), |existing| {
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
        to_address: state.insurer.to_string(),
        amount: vec![coin(withdraw_amount, state.denom.to_string())],
    };

    let res = Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("to", state.insurer)
        .add_attribute("withdraw_amount", withdraw_amount.to_string())
        .add_message(msg);
    Ok(res)
}

pub fn execute_renewal(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    _info: MessageInfo,
    msg: RenewalMsg,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;
    let policy = POLICES.load(deps.storage, &msg.insured_party.to_string())?;

    if msg.insured_party.to_string() != policy.insured_party.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure that the insured oarty is the motus owner
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(msg.insured_party.to_string(), msg.dpr.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
    }

    if response.pubkey.to_string() ==  "".to_string() {
        return Err(ContractError::NoDprForInsuredParty {});
    }

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

    let termination_time = calculate_renewal_termination_time(policy.duration, policy.termination_time);

    let renew_time = msg.duration.mul(3600);
    if !is_policy_eligible_for_renewal(env.block.time.seconds(), renew_time, policy.termination_time) {
        return Err(ContractError::NotEligibleForRenewal {});
    }

    // **TODO: Insert your specific business logic calculations in this section.**

    // **Guidance:**
    // - This section is reserved for implementing your custom business logic calculations tailored to your insurance use case.
    // - Ensure that your calculations align with the objectives and requirements of your insurance smart contract.
    // - Review existing examples and templates within the codebase for inspiration.

    POLICES.update(deps.storage, &msg.insured_party.to_string(), |existing| {
        if let Some(mut res) = existing {

            // Modify the existing policy fields as needed...

            res.premium = msg.premium;
            res.duration = msg.duration;
            res.termination_time = termination_time;
            res.is_active = false;
            // Add more fields if needed
    
            Ok(res)
        } else {
            Err(ContractError::PolicyNotFound {})
        }
    })?;

    let res = Response::new()
        .add_attribute("action", "renew")
        .add_attribute("renewed_by", state.insurer)
        .add_attribute("renew_premium", renewal_premium.to_string());
    Ok(res)
}

pub fn execute_terminate(
    deps: DepsMut<SoarchainQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: TerminateMsg,
) -> Result<Response, ContractError> {

    let policy = POLICES.load(deps.storage, &msg.insured_party.to_string())?;

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    if msg.insured_party.to_string() != policy.insured_party.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure that the insured oarty is the motus owner
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(msg.insured_party.to_string(), msg.dpr.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
    }

    if response.pubkey.to_string() ==  "".to_string() {
        return Err(ContractError::NoDprForInsuredParty {});
    }

    // **TODO: Insert your specific business logic calculations in this section.**

    // **Guidance:**
    // - This section is reserved for implementing your custom business logic calculations tailored to your insurance use case.
    // - Ensure that your calculations align with the objectives and requirements of your insurance smart contract.
    // - Review existing examples and templates within the codebase for inspiration.


    POLICES.update(deps.storage, &policy.insured_party.to_string(), |existing| {
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
        QueryMsg::MotusByAddress { address, dpr} => to_json_binary(&get_motus_by_address(deps, address, dpr)), 
        QueryMsg::PaymentVerification { id } => to_json_binary(&payment_verified(deps, env, id)?),
        QueryMsg::Details { address } => to_json_binary(&query_details(deps, address)?),
        QueryMsg::List {} => to_json_binary(&query_list(deps)?),
    }
}

fn get_motus_by_address(deps: Deps<SoarchainQuery>, index: String, dpr: String) -> MotusByAddressResponse {
    info!("contract-insurance-get_motus_by_address: address {}  Mb/s", index);
    let querier = SoarchainQuerier::new(&deps.querier);
    let response: crate::MotusByAddressResponse = querier.motus_by_address(index, dpr).unwrap();
    info!("contract-insurance-get_motus_by_address: response {}  Mb/s", response.address);

    MotusByAddressResponse {
        address: response.address,
        dpr_id: response.dpr_id,
        pubkey: response.pubkey,
        vin: response.vin,
        dpr: response.dpr.to_string(),
        pid: response.pid,
    }
}

fn payment_verified(deps: Deps<SoarchainQuery>, _env: Env, insured_address: String) -> StdResult<PaymentVerificationResponse> {

    let policy = POLICES.load(deps.storage, &insured_address)?;

    if policy.is_active == true {
        Ok(PaymentVerificationResponse { verified: true })
    } else {
        Ok(PaymentVerificationResponse { verified: false })
    }
}

fn query_details(deps: Deps<SoarchainQuery>, id: String) -> StdResult<DetailsResponse> {
    let policy = POLICES.load(deps.storage, &id)?;

    let details = DetailsResponse {
        id,
        insurer: policy.insurer,
        insured_party: policy.insured_party,
        start_time: policy.start_time,
        premium: policy.premium,
        duration: policy.duration,
        termination_time: policy.termination_time,
        is_active: policy.is_active,
        closed: policy.closed,
    };

    Ok(details)
}

fn query_list(deps: Deps<SoarchainQuery>) -> StdResult<ListResponse> {
    Ok(ListResponse {
        insured_parties: all_policy_insured_parties(deps.storage)?,
    })
}