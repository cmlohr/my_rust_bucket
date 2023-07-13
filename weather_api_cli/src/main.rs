use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = "YOUR API GOES HERE";
    let city = "New Orleans";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        city, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    println!("The current temperature in {} is {}°F", city, response.main.temp);
    println!("It feels like {}°F", response.main.feels_like);
    println!("The high today is {}°F", response.main.temp_max);
    println!("The low today is {}°F", response.main.temp_min);
    println!("The humidity is {}%", response.main.humidity);
    println!("The pressure is {} hPa", response.main.pressure);

    Ok(())
}
