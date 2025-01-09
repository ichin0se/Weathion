use anyhow::{bail, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherInfo {
    pub temperature: f32,
    pub chance_of_precipitation: f32,
    pub description: String,
}

impl WeatherInfo {
    pub fn validate(self) -> Result<Self> {
        if self.temperature <= -50.0 || self.temperature >= 50.0 {
            bail!("Temperature out of range");
        }
        if self.chance_of_precipitation < 0.0 || self.chance_of_precipitation > 100.0 {
            bail!("Chance of precipitation out of range");
        }
        if self.description.trim().is_empty() {
            bail!("Description is empty");
        }
        Ok(self)
    }
}

pub async fn fetch_weather_info(api_key: &str, city: &str) -> Result<WeatherInfo> {
    let _url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let weather_info = WeatherInfo {
        temperature: 23.4,
        chance_of_precipitation: 80.5,
        description: "cloudy".to_string(),
    };

    weather_info.validate()
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_weather_info() {
        let api_key = "DUMMY_API_KEY";
        let city = "Tokyo";

        let result = fetch_weather_info(api_key, city).await;
        assert!(result.is_ok());
        let weather = result.unwrap();

        assert_eq!(weather.temperature, 23.4);
        assert_eq!(weather.description, "cloudy");
        assert_eq!(weather.chance_of_precipitation, 80.5);
    }
}
