use dotenv::dotenv;
use serde::Deserialize;
use serde_json;
use std::io::{BufWriter, Write};
use std::{env, io, process};
use ureq;

#[derive(Debug, Deserialize)]
struct Coord {
    lat: f64,
    lon: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Data {
    main: String,
    description: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    sea_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grnd_level: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Wind {
    speed: f32,
    deg: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    gust: Option<f32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Clouds {
    all: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Weather {
    coord: Coord,
    weather: [Data; 1],
    main: Main,
    #[serde(skip_serializing_if = "Option::is_none")]
    wind: Option<Wind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clouds: Option<Clouds>,
    timezone: i32,
    name: String,
}

fn main() {
    dotenv().ok();
    let args = env::args().collect::<Vec<String>>();
    let key = match env::var("WEATHER_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("WEATHER_API_KEY not found!");
            process::exit(1);
        }
    };
    let city: &str;
    if args.len() == 1 {
        city = "london";
    } else if args.len() == 2 {
        city = &args[1];
    } else {
        eprintln!("sun <city>");
        process::exit(1);
    }

    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={city}&appid={key}");

    let response = ureq::get(&url).call();
    if response.ok() {
        let data = match response.into_string() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to convert request to string!");
                process::exit(1);
            }
        };
        let json_data: [Coord; 1] = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to convert string to json: {e}");
                process::exit(1);
            }
        };
        let lat = json_data[0].lat;
        let lon = json_data[0].lon;
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={key}"
        );
        let response = ureq::get(&url).call();
        if response.ok() {
            let data = match response.into_string() {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Failed to convert request to string!");
                    process::exit(1);
                }
            };
            // println!("{:#?}", data);
            let json_response: Weather = match serde_json::from_str(&data) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to convert string to json: {e}");
                    process::exit(1);
                }
            };

            let stdout = io::stdout().lock();
            let mut buffer = BufWriter::new(stdout);
            let mut logo = r#"   ______  ___  __
  / __/ / / / |/ /
 _\ \/ /_/ /    / 
/___/\____/_/|_/"#
                .to_string();
            logo.push_str("\n\n");
            buffer.write(logo.as_bytes()).unwrap();
            let mut line = format!("City: {}\n", json_response.name);
            buffer.write(line.as_bytes()).unwrap();
            line = format!(
                "Latitude: {} °N\nLongitude: {} °E\n",
                json_response.coord.lat, json_response.coord.lon
            );
            buffer.write(line.as_bytes()).unwrap();
            line = format!(
                "Weather: {}\nDescription: {}\n",
                json_response.weather[0].main, json_response.weather[0].description
            );
            buffer.write(line.as_bytes()).unwrap();
            line = format!(
                "Temperature: {} °C\nFeels like: {} °C\nMinimum temp: {} °C\nMaximum temp: {} °C\nPressure: {} hPa\nHumidity: {}%\n",
                json_response.main.temp - 270.0,
                json_response.main.feels_like - 270.0,
                json_response.main.temp_min - 270.0,
                json_response.main.temp_max - 270.0,
                json_response.main.pressure,
                json_response.main.humidity
            );
            buffer.write(line.as_bytes()).unwrap();
            if json_response.wind.is_some() {
                let wind = json_response.wind.unwrap();
                line = format!("Wind speed: {} m/s\nDegree: {} °\n", wind.speed, wind.deg);
                if wind.gust.is_some() {
                    let gust = format!("Gust: {}\n", wind.gust.unwrap());
                    line.push_str(&gust);
                }
                buffer.write(line.as_bytes()).unwrap();
            }
            if json_response.clouds.is_some() {
                let cloud = json_response.clouds.unwrap();
                line = format!("Clouds: {}%\n", cloud.all);
                buffer.write(line.as_bytes()).unwrap();
            }
        } else {
            eprintln!("Request failed with status code: {}", response.status());
        }
    } else {
        eprintln!("Request failed with status code: {}", response.status());
    }
}
