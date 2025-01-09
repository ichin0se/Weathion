use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotionEvent {
    pub title: String,
    pub date: String,
    pub color: String,
}

pub async fn fetch_notion_events(
    client: &Client,
    token: &str,
    database_id: &str,
) -> Result<Vec<NotionEvent>> {
    let _url = format!("https://api.notion.com/v1/databases/{}/query", database_id);

    /*
    let resp = client
        .post(&_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2022-06-28")
        .json(&serde_json::json!({ /* query params */ }))
        .send()
        .await?;
    let json_value = resp.json::<serde_json::Value>().await?;
    */

    // ダミーデータ
    let dummy_events = vec![
        NotionEvent {
            title: "Meeting with client".to_string(),
            date: "2025-01-10".to_string(),
            color: "LightGreen".to_string(),
        },
        NotionEvent {
            title: "Project deadline".to_string(),
            date: "2025-01-15".to_string(),
            color: "LightRed".to_string(),
        },
    ];

    Ok(dummy_events)
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    #[tokio::test]
    async fn test_fetch_notion_events() {
        let client = Client::new();
        let token = "dummy_token";
        let db_id = "dummy_db_id";

        let result = fetch_notion_events(&client, token, db_id).await;
        assert!(result.is_ok());
        let events = result.unwrap();
        assert!(!events.is_empty());
        assert_eq!(events[0].title, "Meeting with client");
        assert_eq!(events[0].color, "LightGreen".to_string());
    }
}
