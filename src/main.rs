use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use dotenv::dotenv;
use std::env;

mod satellite_info;
use satellite_info::{SatelliteInfo,SatellitePosition};
  
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

    let binding = sat_info.tle.unwrap().replace("\\\"","").replace("\"","");
    let tle_lines: Vec<&str> = binding.split("\\r\\n").collect();

    println!("{:?}",tle_lines);
    
    // code for sgp4 crate
    let elements = sgp4::Elements::from_tle(
        Some("Space Station".to_owned()),
        tle_lines[0].as_bytes(),
        tle_lines[1].as_bytes(),
    )?;
    let constants = sgp4::Constants::from_elements(&elements)?;
    for hours in 0..24 {
        println!("t = {} min", hours * 60);
        let prediction = constants.propagate(sgp4::MinutesSinceEpoch((hours * 60) as f64))?;
        println!("    r = {:?} km", prediction.position);
        println!("    ṙ = {:?} km.s⁻¹", prediction.velocity);
    }

    Ok(())
}

