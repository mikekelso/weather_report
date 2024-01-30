use serde::Deserialize;
use serde::Serialize;
use anyhow :: Result;

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


fn print_current(current: &Current) {
    println!("Current Data:\n");
    println!("Temperature: {}°F   Feels Like: {}°F", current.temp, current.feels_like);
    println!("Cloudiness: {}%", current.clouds);
    println!("Wind Speed: {} mph", current.wind_speed);
   

    
    
    if let Some(weather) = current.weather.first() {
        println!("\nCurrent Overview: {} - {}\n", weather.main, weather.description );
    } else {
        println!("  No weather information available.");
    }

}


fn print_daily(daily: &Daily) {
    println!("Today's Data:\n");
    println!("Summary: {}\n", daily.summary);
    
    

    if let Some(rain) = daily.rain {
        println!("Rain amount today: {} mm", rain);
    } 

    if let Some(snow) = daily.snow {
        println!("Snow amount today: {} mm", snow);
    } 
    println!("Probability of Precipitation: {}%", daily.pop);
    println!("UV Index: {}", daily.uvi);
    println!("Wind Speed: {} mph", daily.wind_speed);
    println!("Cloudiness: {}%", daily.clouds);
   

    println!("\nTemperature Data:\n");
    
    println!("Min Temperature: {}°F", daily.temp.min);
    println!("Max Temperature: {}°F", daily.temp.max);
    println!("Morning Temperature: {}°F   Feels Like: {}°F", daily.temp.morn, daily.feels_like.morn);
    println!("Day Temperature: {}°F   Feels Like: {}°F", daily.temp.day, daily.feels_like.day);
    println!("Evening Temperature: {}°F   Feels Like: {}°F", daily.temp.eve, daily.feels_like.eve);
    

    

    
    }


   




async fn fetch_weather() -> Result<Root>  {
    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat=39.84&lon=-105.04&exclude=hourly,minutely,alerts&appid={APIKEY}&units=imperial");
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    //println!("{}", body);
    let root: Root = serde_json::from_str(&body)?;
    
    
    
    
    Ok(root)
    
}
#[tokio::main]
async fn main() -> Result<()>{
    
    match fetch_weather().await {
        Ok(root) => {
            print_current(&root.current);
            print_daily(&root.daily[0]);
        }
        Err(err) => eprintln!("error: {}", err),
       
        
    }
    Ok(())
}
