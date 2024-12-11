use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use dotenv::dotenv;
use std::env;
  
#[derive(Debug, Deserialize)]
struct SatellitePosition{
    satlatitude: f64,
    satlongitude: f64,
    sataltitude: f64,
    // satellite azimuth with respect to observer's location (degrees)
    azimuth: f64,
    // satellite elevation with respect to observer's location (degrees)
    elevation: f64,
    // satellite right ascension(degrees)
    ra: f64,
    // satellite declination (degrees)
    dec: f64,
    timestamp: f64,
}

#[derive(Debug, Deserialize)]
struct SatelliteInfo{
    satid:String,
    satname:String,
    transactionscount:u64,
    satellite_position:SatellitePosition,
}

#[derive(Debug, Deserialize)]
struct SatelliteInfoList {
    data: Vec<SatelliteInfo>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // json() and text() cannot be called on the same response
    // https://stackoverflow.com/questions/70527320/is-it-possible-to-get-both-the-text-and-the-json-of-a-response-from-reqwest
    
    // n2yo API-Key load 
    dotenv().ok();
    let api_key = env::var("N2YO_API_KEY").expect("N2YO_API_KEY not set");
    let latitude = env::var("LATITUDE").expect("LATITUDE not set");
    let longitude = env::var("LONGITUDE").expect("LONGITUDE not set");

    let api_key_str : String = api_key.parse().expect("Not able to parse API_KEY.");
    let latitude_str : String = latitude.parse().expect("Not able to parse Latitude.");
    let longitude_str : String = longitude.parse().expect("Not able to parse Longitude.");

    // n2yo Example
    // This example is for retrieve Space Station (25544) positions for next 1 seconds. 
    let mut api_endpoint = String::new();
    api_endpoint.push_str("https://api.n2yo.com/rest/v1/satellite/positions/25544/");
    api_endpoint.push_str(&latitude_str);
    api_endpoint.push_str("/");
    api_endpoint.push_str(&longitude_str);
    api_endpoint.push_str("/0/1/&apiKey=");
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
    
    let sat_positions = SatellitePosition {
            satlatitude: parsed_response["positions"][0]["satlatitude"].as_f64().expect("Error when converting f64"),
            satlongitude: parsed_response["positions"][0]["satlongitude"].as_f64().expect("Error when converting f64"),
            sataltitude: parsed_response["positions"][0]["sataltitude"].as_f64().expect("Error when converting f64"),
            azimuth: parsed_response["positions"][0]["azimuth"].as_f64().expect("Error when converting f64"),
            elevation: parsed_response["positions"][0]["elevation"].as_f64().expect("Error when converting f64"),
            ra: parsed_response["positions"][0]["ra"].as_f64().expect("Error when converting f64"),
            dec: parsed_response["positions"][0]["dec"].as_f64().expect("Error when converting f64"),
            timestamp: 0.0 ,
    };

    let sat_info = SatelliteInfo {
            satid: parsed_response["info"]["satid"].to_string(),
            satname: parsed_response["info"]["satname"].to_string(),
            transactionscount: parsed_response["info"]["transactionscount"].as_u64().expect("Error when converting u64"),
            satellite_position: sat_positions,
    };

    println!("{:?}", sat_info);

    Ok(())
}

