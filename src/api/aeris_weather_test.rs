

use crate::aeris_weather;

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

#[test]
fn aeris_access() {
    let data = AerisWeatherRequestData {
        zip: None,
        city: Some("Dallas".to_lowercase()),
        country: Some("USA".to_lowercase())
    };

    match aw!(get(data)) {
        Ok(res) => {
            assert!(res.success)
        },
        Err(_) => {
            assert!(false)
        }
    }

    
}