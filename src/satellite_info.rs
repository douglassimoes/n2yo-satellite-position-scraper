use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SatellitePosition{
    pub satlatitude: f64,
    pub satlongitude: f64,
    pub sataltitude: f64,
    // satellite azimuth with respect to observer's location (degrees)
    pub azimuth: f64,
    // satellite elevation with respect to observer's location (degrees)
    pub elevation: f64,
    // satellite right ascension(degrees)
    pub ra: f64,
    // satellite declination (degrees)
    pub dec: f64,
    pub timestamp: f64,
}

#[derive(Debug, Deserialize)]
pub struct SatelliteInfo{
    pub satid:String,
    pub satname:String,
    pub transactionscount:u64,
    pub tle: Option<String>,
    pub satellite_position:Option<SatellitePosition>,
}

#[derive(Debug, Deserialize)]
pub struct SatelliteInfoList {
    pub data: Vec<SatelliteInfo>,
}

