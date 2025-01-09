use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

// -------------------------------------
// 天気情報の構造体（仮）
// -------------------------------------
#[derive(Debug, Deserialize)]
pub struct WeatherInfo {
    pub temperature: f64,
    pub description: String,
}

// -------------------------------------
// 天気予報APIから情報を取得（ダミー）
// -------------------------------------
pub async fn fetch_weather_info(client: &Client, api_key: &str, city: &str) -> Result<WeatherInfo> {
    let _url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    // 実際にはこんな形で取得する
    // let resp = client.get(&_url).send().await?;
    // let json_data = resp.json::<serde_json::Value>().await?;

    // ダミー値として返す
    let dummy_weather = WeatherInfo {
        temperature: 23.4,
        description: "cloudy".to_string(),
    };

    Ok(dummy_weather)
}

// -------------------------------------
// テストモジュール
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_weather_info() {
        let client = reqwest::Client::new();
        let api_key = "dummy_api_key";
        let city = "dummy_city";

        let result = fetch_weather_info(&client, api_key, city).await;
        assert!(result.is_ok());
        let weather = result.unwrap();
        // テスト用なので、dummyの値を想定通りかチェックする
        assert_eq!(weather.temperature, 23.4);
        assert_eq!(weather.description, "cloudy");
    }
}
