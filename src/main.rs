



mod cli;
mod api;

use api::*;

// use cli::args::CLI;
// use clap::Parser;


// https://api.aerisapi.com/observations/minneapolis,mn?format=json&filter=allstations&limit=10&client_id=[CLIENT_ID]&client_secret=[CLIENT_SECRET]





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = aeris_weather::AerisWeatherRequestData {
        zip: None,
        city: Some("Dallas".to_lowercase()),
        country: Some("Texas".to_lowercase())
    };

    let res = aeris_weather::get(data).await;

    println!("{:#?}", res);

    Ok(())
}



// fn main() {
    
//     let args = CLI::parse();
//     println!("ARGS: {:?}", args);
// }


