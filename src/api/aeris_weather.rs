
pub mod aeris_weather {
    use std::collections::HashMap;
    use serde::{ Deserialize, Serialize };
    use reqwest::header::CONTENT_TYPE;
    use dotenv::dotenv;
    use std::env::var;

    use super::super::super::helpers::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;



    #[derive(Debug, Serialize)]
    pub struct WeatherRequestData {
        pub zip: Option<String>,
        pub city: Option<String>
    }    

    impl Validate for WeatherRequestData {
        fn validate(&self) -> bool {
            match &self.zip {
                Some(_) => {
                    true
                },
                None => {
                    self.city.is_some()
                }
            }
        }
    }

    impl Location for WeatherRequestData {
        fn get_location(&self) -> Result<String> {
            if !self.validate() {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Wrong params")));
            }

            Ok(
                self.zip
                    .clone()
                    .unwrap_or_else(|| format!("{},usa", self.city.as_ref().unwrap()))
            )
        }
    }


    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ObservationObject {
        pub timestamp: Option<f32>,
        pub dateTimeISO: Option<String>,
        pub recTimestamp: Option<f32>,
        pub recDateTimeISO: Option<String>,
        pub tempC: Option<f32>,
        pub tempF: Option<f32>,
        pub dewpointC: Option<f32>,
        pub dewpointF: Option<f32>,
        pub humidity: Option<f32>,
        pub pressureMB: Option<f32>,
        pub pressureIN: Option<f32>,
        pub spressureMB: Option<f32>,
        pub spressureIN: Option<f32>,
        pub altimeterMB: Option<f32>,
        pub altimeterIN: Option<f32>,
        pub windKTS: Option<f32>,
        pub windKPH: Option<f32>,
        pub windMPH: Option<f32>,
        pub windSpeedKTS: Option<f32>,
        pub windSpeedKPH: Option<f32>,
        pub windSpeedMPH: Option<f32>,
        pub windDirDEG: Option<f32>,
        pub windDir: Option<String>,
        pub windGustKTS: Option<f32>,
        pub windGustKPH: Option<f32>,
        pub windGustMPH: Option<f32>,
        pub flightRule: Option<String>,
        pub visibilityKM: Option<f32>,
        pub visibilityMI: Option<f32>,
        pub weather: Option<String>,
        pub weatherShort: Option<String>,
        pub weatherCoded: Option<String>,
        pub weatherPrimary: Option<String>,
        pub weatherPrimaryCoded: Option<String>,
        pub cloudsCoded: Option<String>,
        pub icon: Option<String>,
        pub heatindexC: Option<f32>,
        pub heatindexF: Option<f32>,
        pub windchillC: Option<f32>,
        pub windchillF: Option<f32>,
        pub feelslikeC: Option<f32>,
        pub feelslikeF: Option<f32>,
        pub isDay: bool,
        pub sunrise: Option<f32>,
        pub sunriseISO: Option<String>,
        pub sunset: Option<f32>,
        pub sunsetISO: Option<String>,
        pub snowDepthCM: Option<f32>,
        pub snowDepthIN: Option<f32>,
        pub precipMM: Option<f32>,
        pub precipIN: Option<f32>,
        pub solradWM2: Option<f32>,
        pub solradMethod: Option<String>,
        pub ceilingFT: Option<f32>,
        pub ceilingM: Option<f32>,
        pub light: Option<f32>,
        pub uvi: Option<f32>,
        pub sky: Option<f32>
    }


    #[derive(Debug, Deserialize, Serialize)]
    pub struct Place {
        pub name: Option<String>,
        pub city: Option<String>,
        pub state: Option<String>,
        pub country: Option<String>
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct AerisWeatherResponseData {
        pub id: Option<String>,
        pub dataSource: Option<String>,
        pub place: Place,
        pub obTimestamp: Option<f32>,
        pub obDateTime: Option<String>,
        pub ob: ObservationObject
    }


    #[derive(Debug, Deserialize, Serialize)]
    pub struct AerisWeatherResponseWrapper{
        pub success: bool,
        pub error: Option<HashMap<String, String>>, // 'code' and 'description'
        pub response: Vec<AerisWeatherResponseData>
    }


    pub async fn get(data: WeatherRequestData) -> Result<AerisWeatherResponseWrapper> {
        if !data.validate() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Wrong params")));
        }

        dotenv().ok();

        let client = reqwest::Client::new();

        let api_id = var("AERIS_WEATHER_ID")?;
        let api_secret = var("AERIS_WEATHER_SECRET")?;
        let location = data.get_location()?;

        let uri = format!("https://api.aerisapi.com/observations/{}?format=json&filter=allstations&limit=10&client_id={}&client_secret={}", location, api_id, api_secret);
        
        let res = client.get(uri)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<AerisWeatherResponseWrapper>()
            .await?;

        Ok( res )
    }
}
