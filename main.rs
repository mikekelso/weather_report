use serde::Deserialize;
use serde::Serialize;
use anyhow :: Result;
use std::fs::File;
use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    #[serde(rename = "timezone_offset")]
    pub timezone_offset: i64,
    pub current: Current,
    pub daily: Vec<Daily>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "dew_point")]
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: i64,
    pub visibility: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_deg")]
    pub wind_deg: i64,
    pub weather: Vec<CurrentWeather>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daily {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub moonrise: i64,
    pub moonset: i64,
    #[serde(rename = "moon_phase")]
    pub moon_phase: f64,
    pub summary: String,
    pub temp: Temp,
    #[serde(rename = "feels_like")]
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "dew_point")]
    pub dew_point: f64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_deg")]
    pub wind_deg: i64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    pub weather: Vec<DailyWeather>,
    pub clouds: i64,
    pub pop: f64,
    pub uvi: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeelsLike {
    pub day: f64,
    pub night: f64,
    pub morn:f32,
    pub eve: f32,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyWeather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}


fn print_current(current: &Current,file: &mut File )  {
    writeln!(file, "Current Data:\n");
    writeln!(file, "Temperature: {}°F   Feels Like: {}°F", current.temp, current.feels_like);
    writeln!(file, "Cloudiness: {}%", current.clouds);
    writeln!(file, "Wind Speed: {} mph", current.wind_speed);
   

    
    
    if let Some(weather) = current.weather.first() {
        writeln!(file, "\nCurrent Overview: {} - {}\n", weather.main, weather.description );
    } else {
        writeln!(file, "  No weather information available.");
    }

}


fn print_daily(daily: &Daily, file: &mut File)  {
    writeln!(file, "Today's Data:\n");
    writeln!(file, "Summary: {}\n", daily.summary);
    
    

    if let Some(rain) = daily.rain {
        writeln!(file, "Rain amount today: {} mm", rain);
    } 

    if let Some(snow) = daily.snow {
        writeln!(file, "Snow amount today: {} mm", snow);
    } 
    writeln!(file, "Probability of Precipitation: {}%", daily.pop);
    writeln!(file, "UV Index: {}", daily.uvi);
    writeln!(file, "Wind Speed: {} mph", daily.wind_speed);
    writeln!(file, "Cloudiness: {}%", daily.clouds);
   

    writeln!(file, "\nTemperature Data:\n");
    
    writeln!(file, "Min Temperature: {}°F", daily.temp.min);
    writeln!(file, "Max Temperature: {}°F", daily.temp.max);
    writeln!(file, "Morning Temperature: {}°F   Feels Like: {}°F", daily.temp.morn, daily.feels_like.morn);
    writeln!(file, "Day Temperature: {}°F   Feels Like: {}°F", daily.temp.day, daily.feels_like.day);
    writeln!(file, "Evening Temperature: {}°F   Feels Like: {}°F", daily.temp.eve, daily.feels_like.eve);
    

    

    
    }


   




async fn fetch_weather() -> Result<Root>  {
    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat=39.84&lon=-105.04&exclude=hourly,minutely,alerts&appid=7ed862ccd7e9e2b5bb9808389cba219b&units=imperial");
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    //writeln!(file, "{}", body);
    let root: Root = serde_json::from_str(&body)?;
    
    
    
    
    Ok(root)
    
}
#[tokio::main]
async fn main() -> Result<()>{
    
    let mut file = File::create("output.txt")?;
    match fetch_weather().await {
        Ok(root) => {
            print_current(&root.current, &mut file);
            print_daily(&root.daily[0], &mut file);
        }
        Err(err) => {
            writeln!(file, "error: {}", err)?;
        }
        
    }
    Ok(())
}
