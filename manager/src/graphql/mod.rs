use graphql_client::{GraphQLQuery, Response};

use self::collateral_addresses::CollateralAddressesCollateralAddressesCollection;

type Bytes = String;
type BigInt = String;

#[derive(GraphQLQuery, Copy, Clone, Debug)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/manager_troves.graphql",
    response_derives = "Debug,Clone,Eq,PartialEq,Default",
    normalization = "rust"
)]
pub struct CollateralAddresses;

/// Query subgraph for troves
pub async fn query_troves(
    manager_address: &str,
    url: &str,
) -> Result<Vec<CollateralAddressesCollateralAddressesCollection>, anyhow::Error> {
    let query = collateral_addresses::Variables { manager:manager_address.to_string() , first: 1000, skip: 0 };
    let request_body = CollateralAddresses::build_query(query);
    let client = reqwest::Client::builder()
        .user_agent("manager")
        .build()?;
    let request = client.post(url).json(&request_body);
    let response = request.send().await?.error_for_status()?;
    let response_body: Response<collateral_addresses::ResponseData> = response.json().await?;
    tracing::trace!(
        result = tracing::field::debug(&response_body),
        "Queried result for Troves"
    );

    if let Some(errors) = response_body.errors.as_deref() {
        let e = &errors[0];
        return Err(anyhow::anyhow!("{}", e.message));
    }

    match response_body.data {
        Some(data) => Ok(data.collateral_addresses_collection),
        None => Err(anyhow::anyhow!("No data")),
    }
}

