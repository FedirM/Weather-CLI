

pub mod configuration {
    use std::fs;
    use std::path::Path;
    use serde::{Serialize, Deserialize};


    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub const CONF_PATH: &str = "conf.json";

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum WeatherAPI {
        Open,
        Aeris
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Configuration {
        pub weather_system: WeatherAPI,
        pub zip: Option<String>,
        pub city: Option<String>
    }

    pub const DEFAULT_CONFIG: Configuration = Configuration {
        weather_system: WeatherAPI::Open,
        zip: None,
        city: None
    };

    pub fn get_configuration() -> Result<Configuration> {
        let path = Path::new(CONF_PATH);
        if path.exists() {
            let file = fs::read(path)?;
            let conf: Configuration = serde_json::from_slice(&file)?;
            return Ok(conf);
        }
        return Ok(DEFAULT_CONFIG);
    }

    pub fn update_configuration(conf: &Configuration) -> Result<()> {
        fs::write(Path::new(CONF_PATH), serde_json::to_string(&conf)?)?;
        Ok(())
    }

    pub fn display_config() -> Result<()> {
        
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::presets::UTF8_FULL;
        use comfy_table::*;


        let mut table = Table::new();
        let conf = get_configuration()?;

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Parameter", "Value"])
            .add_row(vec![
                Cell::new("API Name").set_alignment(CellAlignment::Center),
                Cell::new(get_api_name(&conf.weather_system))
            ])
            .add_row(vec![
                Cell::new("City").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}", conf.city.unwrap_or_else(|| "N/A".to_owned())))
            ])
            .add_row(vec![
                Cell::new("ZIP Code").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}", conf.zip.unwrap_or_else(|| "N/A".to_owned())))
            ]);

        
        

        println!("{table}");
        Ok(())
    }

    fn get_api_name(api: &WeatherAPI) -> String {
        match api {
            WeatherAPI::Aeris => "Aeris Weather".to_owned(),
            WeatherAPI::Open => "Open Weather".to_owned()
        }
    }


}