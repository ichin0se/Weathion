use std::str::FromStr;

use crate::notion::NotionEvent;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::calendar::{CalendarEventStore, Monthly},
};
use time::{Date, Month};

pub fn init_calendar_events() -> CalendarEventStore {
    let store = CalendarEventStore::today(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Blue),
    );
    store
}

/// カレンダーを描画する関数
pub fn create_calendar<'a>(
    year: i32,
    month: Month,
    events: &'a CalendarEventStore,
) -> Monthly<'a, &CalendarEventStore> {
    get_month_widget(year, month, events)
}

fn get_month_widget<'a>(
    year: i32,
    month: Month,
    es: &'a CalendarEventStore,
) -> Monthly<'a, &CalendarEventStore> {
    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Green);

    let default_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(50, 50, 50));

    Monthly::new(Date::from_calendar_date(year, month, 1).unwrap(), es)
        .show_surrounding(Style::default().add_modifier(Modifier::DIM))
        .show_weekdays_header(header_style)
        .default_style(default_style)
        .show_month_header(Style::default())
}

pub fn update_calendar_store(store: &mut CalendarEventStore, events: &[NotionEvent]) {
    /*
    for event in events {
        let event_date = time_macros::date!(event.date);
        if let Ok(date) = event_date {
            let style = Style::default()
                .fg(Color::from_str(event.color.as_str()).unwrap())
                .add_modifier(Modifier::BOLD);

            store.add(date, style);
        }
    }
    */
}
