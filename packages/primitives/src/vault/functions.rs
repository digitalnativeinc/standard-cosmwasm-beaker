use cosmwasm_std::{StdResult, QueryRequest, DepsMut};
use osmo_bindings::{SpotPriceResponse, OsmosisQuery, Swap};



pub fn query_spot_price(
    deps: DepsMut<OsmosisQuery>,
    swap: Swap,
    with_swap_fee: bool
) -> StdResult<SpotPriceResponse> {
    
    let spot_price_query = OsmosisQuery::SpotPrice { swap, with_swap_fee };
    let request: QueryRequest<OsmosisQuery> = OsmosisQuery::into(spot_price_query);

    let price = deps.querier.query(&request)?;
    Ok(price)
}