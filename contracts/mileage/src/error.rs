use cosmwasm_std::StdError;
use thiserror::Error;

#[derive( Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized. Ensure that the sender is the owner of the contract")]
    Unauthorized {},

    #[error("Policy is already in use.")]
    AlreadyInUse {},

    #[error("Amount was zero, must be positive")]
    ZeroAmount {},

    #[error("Policy is closed")]
    Closed {},

    #[error("Only Policy creator can close")]
    InvalidUser {},

    #[error("Missing Data. Data must have at least two elements")]
    NoData {},

    #[error("Unauthorized. Ensure that the sender is the insured of the contract")]
    UnauthorizedInsuredParty {},

    #[error("Policy is active. Active policy can not be terminated")]
    Active {},

    #[error("Policy is not active. In Active policy can not be renewal")]
    NoActive {},

    #[error("Policy Not Find.")]
    PolicyNotFound {},

    #[error("Ploicy is not eligible for renewal.")]
    NotEligibleForRenewal {},

    #[error("Calculated premium can not be less that the base rate.")]
    LessPremium {},
}
