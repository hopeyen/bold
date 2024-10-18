use graphql::query_troves;
use std::error::Error;
use tokio;


pub mod graphql;
pub mod util;

// local graph-node GraphQL endpoint
const GRAPHQL_ENDPOINT: &str = "http://localhost:8000/subgraphs/id/Qmd5QMJm1zAyfVWGY4bqeFuXogd5gB38uuL2WM51zez1Bd";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    util::init_tracing().expect("Logger");

    // must be replaced everytime when restarting dev chain... store & get from env somewhere? 
    let manager_address = "0xa6f93a16c8e957eed2c9dbff0985ba8e7ef779bc";

    // Query for a specific trove
    let collaterals = query_troves(manager_address, GRAPHQL_ENDPOINT).await;
    tracing::info!("Got collaterals: {:#?}", collaterals);

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

    Ok(())
}
