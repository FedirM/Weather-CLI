

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub trait Location {
    fn get_location(&self) -> std::result::Result<String, Box<dyn std::error::Error>>;
}

pub trait Validate {
    fn validate(&self) -> bool;
}

#[allow(non_snake_case)]
pub mod Display {
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::*;

    #[derive(Debug)]
    pub struct DisplayData {
        pub city: String,
        pub country: String,
        pub weather: String,
        pub wind_speed: f32,
        pub wind_deg: Option<f32>,
        pub temp: f32, // Celsium
        pub temp_feels: Option<f32>, // Celsium
        pub pressure: f32, // hPA
        pub humidity: f32 // %
    }


    pub fn display(data: DisplayData) {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Parameter", "Value"])
            .add_row(vec![
                Cell::new("Place").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}, {}", data.city, data.country))
            ])
            .add_row(vec![
                Cell::new("Weather").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}", data.weather))
            ])
            .add_row(vec![
                Cell::new("Wind speed").set_alignment(CellAlignment::Center),
                Cell::new(format!("{} km/h", data.wind_speed))
            ]);
        
        if data.wind_deg.is_some() {
            table.add_row(vec![
                Cell::new("Wind degree").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}°", data.wind_deg.unwrap()))
            ]);
        }
            
        table.add_row(vec![
            Cell::new("Temperature").set_alignment(CellAlignment::Center),
            Cell::new(format!("{} C°", data.temp))
        ]);

        if data.temp_feels.is_some() {
            table.add_row(vec![
                Cell::new("Temp feels like").set_alignment(CellAlignment::Center),
                Cell::new(format!("{} C°", data.temp_feels.unwrap()))
            ]);
        }
            
        table.add_row(vec![
                Cell::new("Pressure").set_alignment(CellAlignment::Center),
                Cell::new(format!("{} hPA", data.pressure))
            ])
            .add_row(vec![
                Cell::new("Humidity").set_alignment(CellAlignment::Center),
                Cell::new(format!("{}%", data.humidity))
            ]);

        
        println!("{table}");
    }
}