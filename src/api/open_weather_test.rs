

#[cfg(test)]
mod open_weather_tests {

    use super::super::open_weather::open_weather::*;
    use super::super::super::aw;



    #[test]
    fn open_city_test() {
        let cc_data = WeatherRequestData {
            zip: None,
            city: Some("Dallas".to_lowercase())
        };

        match aw!(get(cc_data)) {
            Ok(res) => {
                assert_eq!(res.code.unwrap(), 200)
            },
            Err(_) => {
                assert!(false)
            }
        }

        
    }
    
    #[test]
    fn open_zip_test() {
        let cc_data = WeatherRequestData {
            zip: Some("75052".to_lowercase()),
            city: None
        };

        match aw!(get(cc_data)) {
            Ok(res) => {
                assert_eq!(res.code.unwrap(), 200)
            },
            Err(_) => {
                assert!(false)
            }
        }

        
    }
}