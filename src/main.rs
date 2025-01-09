mod calendar;
mod notion;
mod ui;
mod weather;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};
use time::{Month, OffsetDateTime};

use notion::NotionEvent;
use ratatui::widgets::calendar::CalendarEventStore;
use weather::WeatherInfo;

pub struct AppState {
    pub notion_events: Vec<NotionEvent>,
    pub weather_info: Option<WeatherInfo>,
    pub calendar_store: CalendarEventStore,
    pub year: i32,
    pub month: Month,
    pub last_update: Instant,
}

impl AppState {
    pub fn new() -> Self {
        let now = OffsetDateTime::now_local().expect("Could not get local time");
        let year: i32 = now.year();
        let month: Month = now.month();

        let calendar_store = calendar::init_calendar_events();

        Self {
            notion_events: vec![],
            weather_info: None,
            last_update: Instant::now(),
            calendar_store,
            year,
            month,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();
    let client = reqwest::Client::new();

    let notion_token = "DUMMY_NOTION_TOKEN";
    let notion_db_id = "DUMMY_NOTION_DB_ID";

    let weather_api_key = "DUMMY_WEATHER_KEY";
    let city = "Tokyo";

    'running: loop {
        terminal.draw(|f| ui::draw(f, &app_state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break 'running;
                        }
                        KeyCode::Char('r') => {
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
        }

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
            app_state.last_update = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

async fn refresh_data(
    client: &reqwest::Client,
    notion_token: &str,
    notion_db_id: &str,
    weather_api_key: &str,
    city: &str,
    app_state: &mut AppState,
) -> anyhow::Result<()> {
    let new_events = notion::fetch_notion_events(client, notion_token, notion_db_id).await?;
    app_state.notion_events = new_events;

    let new_weather = weather::fetch_weather_info(weather_api_key, city).await?;
    app_state.weather_info = Some(new_weather);

    calendar::update_calendar_store(&mut app_state.calendar_store, &app_state.notion_events);

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
