use std::str::FromStr;

use crate::{calendar, AppState, WeatherInfo};
use ratatui::widgets::calendar::CalendarEventStore;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use time::Month;

pub fn draw(frame: &mut Frame, app_state: &AppState) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(size);

    let calendar_area = chunks[0];

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let tasks_area = right_chunks[0];
    let weather_area = right_chunks[1];

    draw_calendar(
        frame,
        calendar_area,
        app_state.year,
        app_state.month,
        &app_state.calendar_store,
    );
    draw_tasks(frame, tasks_area, app_state);
    draw_weather(frame, weather_area, app_state.weather_info.as_ref());
}

fn draw_calendar(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    year: i32,
    month: Month,
    calendar_store: &CalendarEventStore,
) {
    let cal = calendar::create_calendar(year, month, calendar_store);
    frame.render_widget(cal, area);
}

fn draw_tasks(frame: &mut Frame, area: ratatui::layout::Rect, app_state: &AppState) {
    let block = Block::default().title("Tasks").borders(Borders::ALL);

    let items: Vec<ListItem> = app_state
        .notion_events
        .iter()
        .map(|ev| {
            let style = Style::default()
                .fg(Color::from_str(ev.color.as_str()).unwrap())
                .add_modifier(Modifier::BOLD);

            let text = Span::styled(format!("{} - {}", ev.date, ev.title), style);
            ListItem::new(Span::from(text))
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_weather(frame: &mut Frame, area: ratatui::layout::Rect, weather: Option<&WeatherInfo>) {
    let block = Block::default().title("Weather").borders(Borders::ALL);

    let text = match weather {
        Some(info) => format!(
            "Temp: {:.1}°C\nDesc: {}\nPrecip: {:.1}%",
            info.temperature, info.description, info.chance_of_precipitation,
        ),
        None => "No weather data".to_string(),
    };

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
