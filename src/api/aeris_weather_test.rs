

#[cfg(test)]
mod aeris_weather_tests {

    use super::super::aeris_weather::aeris_weather::*;
    use super::super::super::aw;



    #[test]
    fn aeris_city_test() {
        let cc_data = WeatherRequestData {
            zip: None,
            city: Some("Dallas".to_lowercase())
        };

        match aw!(get(cc_data)) {
            Ok(res) => {
                assert!(res.success)
            },
            Err(_) => {
                assert!(false)
            }
        }

        
    }
    
    #[test]
    fn aeris_zip_test() {
        let cc_data = WeatherRequestData {
            zip: Some("75052".to_lowercase()),
            city: None
        };

        match aw!(get(cc_data)) {
            Ok(res) => {
                assert!(res.success)
            },
            Err(_) => {
                assert!(false)
            }
        }

        
    }
}
