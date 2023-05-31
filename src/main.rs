
mod cli;
mod api;
mod helpers;
mod config;

use api::aeris_weather::aeris_weather;
use api::open_weather::open_weather;
use helpers::Display;
use cli::args::*;
use clap::Parser;

use config::configuration::*;

enum WeatherResponse {
    Open(open_weather::OpenWeatherResponseData),
    Aeris(aeris_weather::AerisWeatherResponseWrapper)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CLI::parse();
    
    match &args.command {
        Commands::Configure(cmd) => {
            match cmd.name.to_lowercase().as_str() {
                "open" => configure_api(cmd, WeatherAPI::Open).await,
                "aeris" => configure_api(cmd, WeatherAPI::Aeris).await,
                _ => println!("Wrong config src name. Check help option for more details")
            }
        },
        Commands::Print => display_config().unwrap(),
        Commands::Run(cmd) => {
            if cmd.api.is_some() {
                match cmd.api.as_ref().unwrap().as_str() {
                    "open" => {
                        update_configuration_api(WeatherAPI::Open).await.unwrap();
                    },
                    "aeris" => {
                        update_configuration_api(WeatherAPI::Aeris).await.unwrap();
                    },
                    _ => {}
                }
            }

            display_weather().await;
        }
    }

    Ok(())
}

async fn configure_api(cmd: &ConfigureCommand, api: WeatherAPI) {
    match &cmd.zip {
        Some(zip) => {
            let conf = Configuration {
                zip: Some(zip.to_string()),
                weather_system: api,
                ..DEFAULT_CONFIG
            };

            update_configuration(&conf).unwrap();
        },
        None => {
            if cmd.city.is_some() {
                let conf = Configuration {
                    city: cmd.city.clone(),
                    weather_system: api,
                    ..DEFAULT_CONFIG
                };
                println!("{:#?}", &conf);

                update_configuration(&conf).unwrap();
                println!("Configured!");
            } else {
                panic!("You should set 'zip' or 'city' argument to specify the place. For more details check 'help'.")
            }
        }
    }
}


async fn update_configuration_api(api: WeatherAPI) -> Result<(), Box<dyn std::error::Error>> {
    let mut conf = get_configuration().unwrap();
    conf.weather_system = api;
    update_configuration(&conf)?;
    Ok(())
}

async fn display_weather() {

    match weather_request().await.unwrap() {
        WeatherResponse::Open(weather) => {
            if weather.code.unwrap() != 200 {
                panic!("Something went wrong with OpenWeather API");
            }

            let display_data = Display::DisplayData{
                city: weather.name.unwrap(),
                country: "USA".to_string(),
                weather: weather.weather.as_ref().unwrap().get(0).unwrap().description.clone().unwrap(),
                wind_speed: weather.wind.as_ref().unwrap().speed.unwrap(),
                wind_deg: weather.wind.as_ref().unwrap().deg,
                temp: weather.main.as_ref().unwrap().temp.unwrap(),
                temp_feels: weather.main.as_ref().unwrap().feels_like,
                pressure: weather.main.as_ref().unwrap().pressure.unwrap(),
                humidity: weather.main.as_ref().unwrap().humidity.unwrap(),
            };

            Display::display(display_data);
        },

        WeatherResponse::Aeris(weather) => {
            if !weather.success {
                panic!("Something went wrong with AerisWeather API");
            }
            let weather = weather.response.get(0).unwrap();
            let display_data = Display::DisplayData{
                city: weather.place.city.as_ref().unwrap().to_string(),
                country: weather.place.country.as_ref().unwrap().to_string(),
                weather: weather.ob.weather.as_ref().unwrap().to_string(),
                wind_speed: weather.ob.windSpeedKPH.unwrap(),
                wind_deg: weather.ob.windDirDEG,
                temp: weather.ob.tempC.unwrap(),
                temp_feels: weather.ob.feelslikeC,
                pressure: weather.ob.pressureMB.unwrap(),
                humidity: weather.ob.humidity.unwrap(),
            };

            Display::display(display_data);
        }
    }
}



async fn weather_request() -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    match get_configuration() {
        Ok(conf) => {
            if conf.city.is_none() && conf.zip.is_none() {
                panic!("You should configure cli before usage. Call `help` for more details.");
            } else {
                match conf.weather_system {
                    WeatherAPI::Aeris => {
                        let req_data = aeris_weather::WeatherRequestData {
                            zip: conf.zip.clone(),
                            city: conf.city.clone()
                        };
        
                        let data = aeris_weather::get(req_data).await?;
                        Ok(WeatherResponse::Aeris(data))
                    },
                    WeatherAPI::Open => {
                        let req_data = open_weather::WeatherRequestData {
                            zip: conf.zip.clone(),
                            city: conf.city.clone()
                        };
        
                        let data = open_weather::get(req_data).await?;
                        Ok(WeatherResponse::Open(data))
                    }
                }
            }
        },
        Err(err) => {
            panic!("{}", err.to_string());
        }
    }
}


