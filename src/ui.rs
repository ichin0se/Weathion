use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::AppState;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState) {
    let size = f.size();

    // 画面を縦に２分割 (上: 天気, 下: カレンダー)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(size);

    // ========== 天気表示 ==========
    let weather_block = Block::default().title("Weather").borders(Borders::ALL);
    let weather_text = match &app_state.weather_info {
        Some(info) => format!(
            "Temp: {:.1}°C\nDesc: {}",
            info.temperature, info.description
        ),
        None => "No data".to_string(),
    };
    let weather_paragraph = Paragraph::new(weather_text).block(weather_block);
    f.render_widget(weather_paragraph, chunks[0]);

    // ========== カレンダー表示 ==========
    let events_block = Block::default().title("Calendar").borders(Borders::ALL);
    let items: Vec<ListItem> = app_state
        .notion_events
        .iter()
        .map(|ev| {
            ListItem::new(Spans::from(vec![Span::raw(format!(
                "{} - {}",
                ev.date, ev.title
            ))]))
        })
        .collect();

    let events_list = List::new(items)
        .block(events_block)
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(events_list, chunks[1]);
}

// -------------------------------------
// テストモジュール
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::terminal::Terminal;

    #[test]
    fn test_draw_ui() {
        // RatatuiのTestBackendを使った描画テストの例
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        let app_state = AppState::new();

        terminal
            .draw(|f| {
                draw_ui(f, &app_state);
            })
            .unwrap();

        // 出力内容のアサートなどは省略
        // 実際にはsnapshotテストや画面の一部文言チェックなどを行うケースもある
    }
}
