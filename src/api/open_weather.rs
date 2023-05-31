
pub mod open_weather {
    
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
            return self.city.is_some() || self.zip.is_some();
        }
    }

    impl Location for WeatherRequestData {
        fn get_location(&self) -> Result<String> {
            if !self.validate() {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Wrong params.")));
            }
            
            match &self.zip {
                Some(zip) => {
                    Ok(format!("{}", zip))
                }, 
                None => {
                    Ok(format!("{},usa", &self.city.clone().unwrap()))
                }
            }
        }
    }



    #[derive(Debug, Deserialize, Serialize)]
    pub struct Weather {
        pub main: Option<String>,
        pub description: Option<String>
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Main {
        pub temp: Option<f32>,
        pub feels_like: Option<f32>,
        pub temp_min: Option<f32>,
        pub temp_max: Option<f32>,
        pub humidity: Option<f32>,
        pub pressure: Option<f32>
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Wind {
        pub speed: Option<f32>,
        pub deg: Option<f32>
    }


    #[derive(Debug, Deserialize, Serialize)]
    pub struct OpenWeatherResponseData {
        pub code: Option<u16>,
        pub name: Option<String>, // City name
        pub weather: Option<Vec<Weather>>,
        pub main: Option<Main>,
        pub wind: Option<Wind>

    }


    pub async fn get(data: WeatherRequestData) -> Result<OpenWeatherResponseData> {
        if !data.validate() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Wrong params")));
        }

        dotenv().ok();

        let client = reqwest::Client::new();

        let api_id = var("OPEN_WEATHER_ID")?;
        let location = data.get_location()?;

        let uri = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", location, api_id);
        
        let res = client.get(uri)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let code = res.status().as_u16();

        let mut data = res
            .json::<OpenWeatherResponseData>()
            .await?;

        data.code = Some(code);


        Ok( data )
    }

}