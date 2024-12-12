use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use dotenv::dotenv;
use std::env;
 
use satellite_info::{SatelliteInfo, SatellitePosition,SatelliteInfoList};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // json() and text() cannot be called on the same response
    // https://stackoverflow.com/questions/70527320/is-it-possible-to-get-both-the-text-and-the-json-of-a-response-from-reqwest
    
    // n2yo API-Key load 
    dotenv().ok();
    let api_key = env::var("N2YO_API_KEY").expect("N2YO_API_KEY not set");
    let api_key_str : String = api_key.parse().expect("Not able to parse API_KEY.");

    // n2yo Example
    // This example is for retrieve Space Station (25544) positions for next 1 seconds. 
    let mut api_endpoint = String::new();
    api_endpoint.push_str("https://api.n2yo.com/rest/v1/satellite/tle/25544&apiKey=");
    api_endpoint.push_str(&api_key);
    let response = reqwest::get(api_endpoint)
        .await?
        .text()
        .await?;

    let parsed_response: Value = match serde_json::from_str(&response) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return Err(err.into());
        } 
    };
    

    let sat_info = SatelliteInfo {
            satid: parsed_response["info"]["satid"].to_string(),
            satname: parsed_response["info"]["satname"].to_string(),
            transactionscount: parsed_response["info"]["transactionscount"].as_u64().expect("Error when converting u64"),
            tle: Some(parsed_response["tle"].to_string()),
            satellite_position: None,
    };

    println!("{:?}", sat_info);

    Ok(())
}

