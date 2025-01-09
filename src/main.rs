use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use reqwest::Client;
use tokio::time::sleep;

// 自作モジュールを参照
mod notion;
mod ui;
mod weather;

// アプリ状態を保持する構造体
pub struct AppState {
    pub notion_events: Vec<notion::NotionEvent>,
    pub weather_info: Option<weather::WeatherInfo>,
    pub last_update: Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            notion_events: vec![],
            weather_info: None,
            last_update: Instant::now(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let notion_token = "YOUR_NOTION_TOKEN";
    let notion_db_id = "YOUR_NOTION_DB_ID";
    let weather_api_key = "YOUR_WEATHER_API_KEY";
    let city = "Tokyo";

    // HTTP Client
    let client = Client::new();

    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // AppState Initialize
    let mut app_state = AppState::new();

    // Main Loop
    'running: loop {
        // Key Event
        if event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        // Quit by "Q"
                        break 'running;
                    }
                    KeyCode::Char('r') => {
                        // reload by "R"
                        refresh_data(
                            &client,
                            notion_token,
                            notion_db_id,
                            weather_api_key,
                            city,
                            &mut app_state,
                        )
                        .await?;
                    }
                    _ => {}
                }
            }
        }

        // Refresh
        if app_state.last_update.elapsed() > Duration::from_secs(10) {
            refresh_data(
                &client,
                notion_token,
                notion_db_id,
                weather_api_key,
                city,
                &mut app_state,
            )
            .await?;
        }

        // Draw
        terminal.draw(|f| ui::draw_ui(f, &app_state))?;
    }

    // Quit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

// -------------------------------------
// Fetch
// -------------------------------------
async fn refresh_data(
    client: &Client,
    notion_token: &str,
    notion_db_id: &str,
    weather_api_key: &str,
    city: &str,
    app_state: &mut AppState,
) -> anyhow::Result<()> {
    // Update Notion Events
    let new_events = notion::fetch_notion_events(client, notion_token, notion_db_id).await?;
    app_state.notion_events = new_events;

    // Update Weather
    let new_weather = weather::fetch_weather_info(weather_api_key, city).await?;
    app_state.weather_info = Some(new_weather);

    // Update Timestamp
    app_state.last_update = Instant::now();

    Ok(())
}

// -------------------------------------
// Test
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_init() {
        let app_state = AppState::new();
        assert!(app_state.notion_events.is_empty());
        assert!(app_state.weather_info.is_none());
    }
}
