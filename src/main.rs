use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

//fucntion to get the weather info from open weather map
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}",
        city, country_code, api_key,
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

//fucntion to display the weather info
fn display_weather_info(response: &WeatherResponse) {
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.main.wind_speed;

    //inputing weather info into a string
    let weather_text: String = format!(
        "Weather in {}:{}
        >Temperature:{:.1}°C,
        >Humidity:{:.1}%,
        >Pressure:{:.1}hPa
        >Wind Speed:{:.1}m/s",
        response.name,
        description,
        //write a function to display an emoji based on the weather (for later)
        temperature,
        humidity,
        pressure,
        wind_speed
    );
    //coloring the text based on temp
    let temp_colored_text: coloredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
}

fn main() {
    println!("{}", "Welcome to another weather app".bright_yellow());
    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::Stdin().read_line(&mut city).expect("invalid input");
        let city: &str = city.trim();
    }
}
