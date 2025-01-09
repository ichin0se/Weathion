use anyhow::{bail, Result};
use reqwest::Client;
use serde::Deserialize;

// -------------------------------------
// Struct of the Weather Infomation
// -------------------------------------
#[derive(Debug, Deserialize)]
pub struct WeatherInfo {
    pub temperature: f64,
    pub chance_of_precipitation: f32,
    pub description: String,
}

impl WeatherInfo {
    pub fn validate(self) -> Result<Self> {
        if self.temperature <= -50.0 || 50.0 <= self.temperature {
            bail!("Temperature out of range");
        }
        if self.chance_of_precipitation < 0.0 || 100.0 < self.chance_of_precipitation {
            bail!("Chance of Precipitation out of range");
        }

        if self.description.trim().is_empty() {
            bail!("Description is empty");
        }
        Ok(self)
    }
}

// -------------------------------------
// Fetch Weather Data from API
// -------------------------------------
pub async fn fetch_weather_info(api_key: &str, city: &str) -> Result<WeatherInfo> {
    let _url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    // let resp = client.get(&_url).send().await.map_err(|_| "Request failed")?;
    // let json_data = resp.json::<serde_json::Value>().await.map_err(|_| "JSON parse failed")?;
    // let weather_info = WeatherInfo {
    //     temperature: json_data["main"]["temp"].as_f64().unwrap_or_default(),
    //     chance_of_precipitation: some_value_from_json,
    //     description: some_string_from_json,
    // };

    let weather_info = WeatherInfo {
        temperature: 23.4,
        chance_of_precipitation: 80.5,
        description: "cloudy".to_string(),
    };

    weather_info.validate()
}

// -------------------------------------
// Test
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_weather_info() {
        //let client = reqwest::Client::new();
        let api_key = "dummy_api_key";
        let city = "dummy_city";

        let result = fetch_weather_info(api_key, city).await;
        assert!(result.is_ok());
        let weather = result.unwrap();
        // test dummy
        assert_eq!(weather.temperature, 23.4);
        assert_eq!(weather.description, "cloudy");
    }
}
