use ethers_core::types::H160;
use graphql::query_troves;
use std::{error::Error, str::FromStr};
use tokio;

struct InterestBatchManager {
    min_interest_rate: u128,
    max_interest_rate: u128,
    min_interest_rate_change_period: u128,
}


pub mod graphql;
pub mod util;
pub mod transactor;

// local graph-node GraphQL endpoint
const GRAPHQL_ENDPOINT: &str = "http://localhost:8000/subgraphs/id/QmSLQ9DPftZin9SV3zAqzSMsK7wGCxWs6rNiRKrwcx7M9g";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    util::init_tracing().expect("Logger");

    // must be replaced everytime when restarting dev chain... store & get from env somewhere? 
    let trove_manager = "0x1b43431ab79cefbd6b4779f6294272e66dbebd75";
    let batch_manager = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720";

    // Query for a specific trove
    let collaterals = query_troves(trove_manager, GRAPHQL_ENDPOINT).await;
    tracing::info!("Got collaterals: {:#?}", collaterals);

    let transactor = transactor::Transactor::new().await.expect("Initialize transactor");
    let result = transactor.get_interest_batch_manager(batch_manager).await;

    tracing::info!(res = tracing::field::debug(&result), "result");
    // // Calculate DIFR for the managed troves
    // let difr = calculate_difr(&troves);
    // tracing::info!("Got DIFR {:?}", difr);



    // // Check if interest rate for the troves is within the acceptable range
    // let is_within_range = check_interest_rate_range(&troves, &difr);
    // tracing::info!("Is interest rate within range? {:?}", is_within_range);


    // // If the interest rate is not within the acceptable range, trigger a rebalance
    // if !is_within_range {
    //     adjust_trove_interest_rate(&troves, &difr);
    // }


    let new_rate: u128 = 12000000000000000;
    let _ = transactor.set_annual_interest_rate(new_rate).await;



    Ok(())
}
