use cosmwasm_schema::write_api;
use mars_swapper_osmosis::route::OsmosisRoute;
use mars_types::swapper::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<OsmosisRoute>,
        query: QueryMsg,
    }
}
