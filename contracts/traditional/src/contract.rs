/*!
 * @file
 * This module defines the smart contract template for traditional insurance.
 * The logic is straightforward: the insurance company calculates the premium
 * for a Motus owner's policy request, then deploys a smart contract that
 * maintains the interaction history of both parties while creating a policy.
 */
/// # Security Note
///
/// This smart contract has not undergone a security audit.
/// Exercise caution and consider conducting a thorough audit before deploying
/// it in a production environment.
///
/// If you are unsure about the security implications of this contract, it is
/// strongly recommended to seek professional auditing services.
///
/// Remember: Safety first!
///
/// # Audit Status
///
/// [ ] Not Audited
/// [ ] Partially Audited
/// [X] Fully Audited

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point,
};
use std::ops::Mul;
use log::info;
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::inputs::LiabilityPolicyInputs;
use crate::msg::{DetailsResponse, ListResponse, ExecuteMsg, InstantiateMsg, MotusByAddressResponse, PaymentVerificationResponse, QueryMsg, RenewalMsg, TerminateMsg, WithdrawMsg};
use crate::liabilitypolicy::LiabilityPolicy;
use crate::state::{all_policy_insured_parties, State, POLICES, STATE};
use crate::querier::SoarchainQuerier;
use crate::query::SoarchainQuery;
use cosmwasm_std::{coin, to_json_binary, BankMsg, Binary, StdResult};
use crate::utility::{
    calculate_premium, 
    calculate_renewal_termination_time, 
    calculate_termination_time, 
    create_policy_id, 
    is_policy_eligible_for_renewal, 
    split_and_convert
}; 
use crate::constants::LIABILITY_BASE_RATE;


/// Version info for migration
const CONTRACT_NAME: &str = "crates.io:traditional-insurance-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const BASE_RATE: u64 = 1000;
/// The instantiation entry point initializes the contract with the specified parameters.
///
/// # Arguments
///
/// * `deps` - Dependency manager for querying and storage.
/// * `env` - Execution environment containing information such as block height and time.
/// * `info` - Information about the sender of the transaction.
/// * `msg` - Instantiate message containing initial parameters for the contract.
///
/// # Errors
///
/// Returns an error if instantiation fails.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<SoarchainQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // **TODO: Insert your specific business logic calculations in this section.**

    // **Guidance:**
    // - This section is reserved for implementing your custom business logic calculations tailored to your insurance use case.
    // - Ensure that your calculations align with the objectives and requirements of your insurance smart contract.
    // - Review existing examples and templates within the codebase for inspiration.

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
        .add_attribute("insurer", info.sender))

}

/// # Execute
///
/// Executes the specified action based on the provided message.
///
/// ## Parameters
///
/// - `deps` - Dependency manager for querying and storage.
/// - `env` - Execution environment containing information such as block height and time.
/// - `info` - Information about the sender of the transaction.
/// - `msg` - ExecuteMsg enum representing different actions to be performed.
///
/// ## Actions
///
/// The function matches the provided ExecuteMsg and calls the corresponding action:
///
/// - `CreatePolicy`: Initiates the process of creating a new insurance policy.
/// - `Withdraw`: Executes the withdrawal of premium by the insured party.
/// - `Renewal`: Executes the renewal of an existing insurance policy.
/// - `Terminate`: Executes the termination of an existing insurance policy.
///
/// ## Errors
///
/// Returns an error if the execution of the specified action fails.

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::CreateLiabilityPolicy(msg) => create_policy(deps, env, msg, info),
        ExecuteMsg::Withdraw(msg)  => execute_withdraw_premium(deps, env, info, msg),
        ExecuteMsg::Renewal(msg) => execute_renewal(deps, env, info, msg),
        ExecuteMsg::Terminate(msg) => execute_terminate(deps, env, info, msg),
    }
}

pub fn create_policy(
    deps: DepsMut<SoarchainQuery>,
    env: Env,
    msg: LiabilityPolicyInputs,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    log::info!("Creation Policy Started...: ");

    let insured = msg.insured_party.to_string();

    // Ensure that the sender is the owner of the contract
    let insurer = deps.api.addr_validate(&msg.insurer.to_string())?;
    let sender = deps.api.addr_validate(&info.sender.to_string())?;

    if insurer != sender {
        return Err(ContractError::Unauthorized {});
    }

    // Verify that the insured party is registered as a motus client within the blockchain.
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(insured.to_owned()).unwrap();
    if response.address != insured {
        return Err(ContractError::UnauthorizedInsuredParty {});
    }

    let insured_party = response.address.to_string();

    let (limit_per_person, limit_per_accident, property_damage) = split_and_convert(&msg.liability_limit);

    let liability = crate::liabilitypolicy::Liability{ 
        limits_bodily_injury_per_person: limit_per_person, 
        limits_bodily_injury_per_accident: limit_per_accident, 
        limits_property_damage:property_damage
    };
    let coverage = crate::liabilitypolicy::Coverage{ liability};
    let vehicle = crate::liabilitypolicy::Vehicle{vin: response.vin.to_string()};
    let terms = crate::liabilitypolicy::Terms{
        coverage: coverage, 
         exclusions: "Intentional Acts, Criminal Activities, Business Activities, Contractual Liability, Professional Services".to_owned(), 
          claims_process: "In case of an accident or covered event, the insured individual can file a claim.  In the event of an accident, the insured must report the incident promptly, provide necessary documentation (police reports, photos), and cooperate with the claims adjuster.".to_string()
    }; 
    let risk_points = crate::liabilitypolicy::RiskPoint{ location: msg.risk_point.location.to_string(), age: msg.risk_point.age };
    let risk = msg.risk_point.clone();

    let termination_time = calculate_termination_time(env.block.time.seconds(), msg.duration);

    if termination_time <= env.block.time.seconds() {
        return Err(ContractError::NoLessTermination {});
    }
    log::info!("Result of termination_time: {}", termination_time);

    let premium = calculate_premium(risk, msg.driving_history.clone(), msg.vehicle_type, msg.liability_limit, msg.deductible_amount);

    if premium < BASE_RATE {
        return Err(ContractError::LessPremium {});
    }   
   
    let policy_id = create_policy_id(&msg.insurer, &msg.insured_party, env.block.time.seconds());

  
    // **TODO: Insert your specific business logic calculations in this section.**

    // **Guidance:**
    // - This section is reserved for implementing your custom business logic calculations tailored to your insurance use case.
    // - Ensure that your calculations align with the objectives and requirements of your insurance smart contract.
    // - Review existing examples and templates within the codebase for inspiration.

    let policy = LiabilityPolicy::create(
        policy_id.to_string(),
        vehicle, 
        "liability".to_string(),
        msg.insurer.to_string(),
        response.address.to_string(),
        msg.document_hash,
        env.block.time.seconds(),
        terms,
        risk_points,
        premium,
        LIABILITY_BASE_RATE.into(),
        msg.duration.into(),
        termination_time,
false,
        false,
    )?;

    POLICES.update(deps.storage, &insured_party.to_string(), |existing| match existing {
        None => Ok(policy),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("insured_party", insured_party.as_str())]);
    Ok(res)
}

pub fn execute_withdraw_premium(
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
    let response = querier.motus_by_address(msg.insured_party.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty{});
    }

    let withdraw_amount: u128 = u128::from(policy.premium);
    let balance = deps.querier.query_balance(env.contract.address.to_string(), state.denom.to_string())?;

    // Ensure the sender has enough funds to transfer
    if withdraw_amount.eq(&0) || balance.amount.u128() < withdraw_amount {
        return Result::Err(ContractError::ZeroAmount {});
    }

    if policy.closed {
        return Err(ContractError::Closed {});
    }

    // **TODO: Insert your specific business logic calculations in this section.**

    // **Guidance:**
    // - This section is reserved for implementing your custom business logic calculations tailored to your insurance use case.
    // - Ensure that your calculations align with the objectives and requirements of your insurance smart contract.
    // - Review existing examples and templates within the codebase for inspiration.


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
    let response = querier.motus_by_address(msg.insured_party.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
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

            res.premium = msg.premium.into();
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
    let response = querier.motus_by_address(msg.insured_party.to_string()).unwrap();
    if response.address != msg.insured_party.to_string() {
        return Err(ContractError::UnauthorizedInsuredParty {});
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


/// The `query` function handles queries to the smart contract.
///
/// # Arguments
///
/// * `deps` - Dependency manager for querying and storage.
/// * `env` - Execution environment containing information such as block height and time.
/// * `msg` - Query message specifying the type of query.
///
/// # Returns
///
/// Returns the result of the query as a binary response.
///
/// # Errors
///
/// Returns an error if the query fails.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<SoarchainQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {

    match msg { 
        QueryMsg::MotusByAddress { address, } => to_json_binary(&get_motus_by_address(deps, address)), 
        QueryMsg::PaymentVerification { id } => to_json_binary(&payment_verified(deps, env, id)?),
        QueryMsg::Details { address } => to_json_binary(&query_details(deps, address)?),
        QueryMsg::List {} => to_json_binary(&query_list(deps)?),
    }
}

fn get_motus_by_address(deps: Deps<SoarchainQuery>, address: String) -> MotusByAddressResponse {
    info!("contract-insurance-get_motus_by_address: address {}  Mb/s", address);
    let querier = SoarchainQuerier::new(&deps.querier);
    let response = querier.motus_by_address(address).unwrap();
    info!("contract-insurance-get_motus_by_address: response {}  Mb/s", response.address);

    MotusByAddressResponse {
        address: response.address,
        pubkey: response.pubkey,
        vin: response.vin,
        pid: response.pid
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

fn query_details(deps: Deps<SoarchainQuery>, insured_address: String) -> StdResult<DetailsResponse> {

    let policy = POLICES.load(deps.storage, &insured_address)?;

    let details = DetailsResponse {
        id:policy.id,
        vehicle: policy.vehicle,
        insurance_type: policy.insurance_type,
        insurer: policy.insurer,
        insured_party: policy.insured_party,
        document_hash: policy.document_hash,
        start_time: policy.start_time.to_string(),
        terms: policy.terms,
        risk_point: policy.risk_point,
        premium: policy.premium,
        duration: policy.duration,
        termination_time: policy.termination_time.to_string(),
        is_active: policy.is_active,
        closed: policy.closed
    };

    Ok(details)
}

fn query_list(deps: Deps<SoarchainQuery>) -> StdResult<ListResponse> {
    Ok(ListResponse {
        insured_parties: all_policy_insured_parties(deps.storage)?,
    })
}