use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::query::{
    MotusByAddressResponse, SoarchainQuery,
};

/// This is a helper wrapper to easily use our custom queries
pub struct SoarchainQuerier<'a> {
    querier: &'a QuerierWrapper<'a, SoarchainQuery>,
}

impl<'a> SoarchainQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<SoarchainQuery>) -> Self {
        SoarchainQuerier { querier }
    }

    pub fn motus_by_address(
        &self,
        address: String,
        dpr: String,
    ) -> StdResult<MotusByAddressResponse> {
        let motus_by_address_query = SoarchainQuery::MotusByAddress {
            address,
            dpr,
        };
        let request: QueryRequest<SoarchainQuery> = SoarchainQuery::into(motus_by_address_query);
        self.querier.query(&request)
    }
}



