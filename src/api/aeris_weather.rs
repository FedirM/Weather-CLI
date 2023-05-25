
use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use reqwest::header::CONTENT_TYPE;
use dotenv::dotenv;
use std::env::var;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Serialize)]
pub struct AerisWeatherRequestData {
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>
}

impl AerisWeatherRequestData {

    fn validate(&self) -> bool {
        match &self.zip {
            Some(_) => {
                true
            },
            None => {
                self.city.is_some() && self.country.is_some()
            }
        }
    }

    fn get_location(&self) -> String {
        self.zip
            .clone()
            .unwrap_or_else(|| format!("{},{}", self.city.as_ref().unwrap(), self.country.as_ref().unwrap()))
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ObservationObject {
    timestamp: Option<f64>,
    dateTimeISO: Option<String>,
    recTimestamp: Option<f64>,
    recDateTimeISO: Option<String>,
    tempC: Option<f64>,
    tempF: Option<f64>,
    dewpointC: Option<f64>,
    dewpointF: Option<f64>,
    humidity: Option<f64>,
    pressureMB: Option<f64>,
    pressureIN: Option<f64>,
    spressureMB: Option<f64>,
    spressureIN: Option<f64>,
    altimeterMB: Option<f64>,
    altimeterIN: Option<f64>,
    windKTS: Option<f64>,
    windKPH: Option<f64>,
    windMPH: Option<f64>,
    windSpeedKTS: Option<f64>,
    windSpeedKPH: Option<f64>,
    windSpeedMPH: Option<f64>,
    windDirDEG: Option<f64>,
    windDir: Option<String>,
    windGustKTS: Option<f64>,
    windGustKPH: Option<f64>,
    windGustMPH: Option<f64>,
    flightRule: Option<String>,
    visibilityKM: Option<f64>,
    visibilityMI: Option<f64>,
    weather: Option<String>,
    weatherShort: Option<String>,
    weatherCoded: Option<String>,
    weatherPrimary: Option<String>,
    weatherPrimaryCoded: Option<String>,
    cloudsCoded: Option<String>,
    icon: Option<String>,
    heatindexC: Option<f64>,
    heatindexF: Option<f64>,
    windchillC: Option<f64>,
    windchillF: Option<f64>,
    feelslikeC: Option<f64>,
    feelslikeF: Option<f64>,
    isDay: bool,
    sunrise: Option<f64>,
    sunriseISO: Option<String>,
    sunset: Option<f64>,
    sunsetISO: Option<String>,
    snowDepthCM: Option<f64>,
    snowDepthIN: Option<f64>,
    precipMM: Option<f64>,
    precipIN: Option<f64>,
    solradWM2: Option<f64>,
    solradMethod: Option<String>,
    ceilingFT: Option<f64>,
    ceilingM: Option<f64>,
    light: Option<f64>,
    uvi: Option<f64>,
    sky: Option<f64>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    name: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct AerisWeatherResponseData {
    id: Option<String>,
    dataSource: Option<String>,
    place: Place,
    obTimestamp: Option<f64>,
    obDateTime: Option<String>,
    ob: ObservationObject
}


#[derive(Debug, Deserialize, Serialize)]
pub struct AerisWeatherResponseWrapper{
    success: bool,
    error: Option<HashMap<String, String>>, // 'code' and 'description'
    response: Vec<AerisWeatherResponseData>
}


pub async fn get(data: AerisWeatherRequestData) -> Result<AerisWeatherResponseWrapper> {
    if !data.validate() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Wrong params")));
    }

    dotenv().ok();

    let client = reqwest::Client::new();

    let api_id = var("AERIS_WEATHER_ID")?;
    let api_secret = var("AERIS_WEATHER_SECRET")?;

    let uri = format!("https://api.aerisapi.com/observations/{}?format=json&filter=allstations&limit=10&client_id={}&client_secret={}", data.get_location(), api_id, api_secret);
    
    let res = client.get(uri)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<AerisWeatherResponseWrapper>()
        .await?;

    Ok( res )
}
