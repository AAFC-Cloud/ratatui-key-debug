use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

fn main() {
    let mut terminal = ratatui::init();

    terminal.clear().unwrap();
    let mut key_events: Vec<(Instant, event::KeyEvent)> = Vec::new();
    loop {
        if event::poll(std::time::Duration::from_millis(50)).unwrap()
            && let Event::Key(key) = event::read().unwrap()
        {
            key_events.push((Instant::now(), key));
        }

        // if esc hit 3 times in last 1.5 seconds, exit
        if key_events
            .iter()
            .rev()
            .filter(|(time, key)| {
                key.code == ratatui::crossterm::event::KeyCode::Esc
                    && time.elapsed().as_millis() < 1500
                    && key.kind == KeyEventKind::Press
            })
            .count()
            >= 3
        {
            break;
        }

        terminal
            .draw(|f| {
                Paragraph::new(
                    key_events
                        .iter()
                        .rev()
                        .take(f.area().height as usize - 2)
                        .map(|(time, key)| {
                            Line::from(format!(
                                "{:>14} ago: {:?}",
                                humantime::format_duration(Duration::from_millis(
                                    time.elapsed().as_millis() as u64
                                ))
                                .to_string(),
                                key
                            ))
                        })
                        .collect::<Vec<_>>(),
                )
                .block(
                    Block::bordered()
                        .title("Key Events (Hit Esc 3 times to exit)")
                        .border_set(symbols::border::ROUNDED)
                        .padding(Padding::uniform(1)),
                )
                .render(f.area(), f.buffer_mut());
            })
            .unwrap();
    }
    ratatui::restore();
}
