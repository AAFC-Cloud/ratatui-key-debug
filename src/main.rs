use std::{io::stdout, time::{Duration, Instant}};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
        execute,
    },
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

fn main() {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture).unwrap();

    terminal.clear().unwrap();
    let mut key_events: Vec<(Instant, event::KeyEvent)> = Vec::new();
    let mut visible_events: Vec<(Instant, String)> = Vec::new();
    loop {
        if event::poll(std::time::Duration::from_millis(50)).unwrap()
            && let event = event::read().unwrap()
        {
            let now = Instant::now();
            match event {
                Event::Key(key) => {
                    key_events.push((now, key));
                    visible_events.push((now, format!("{key:?}")));
                }
                Event::Mouse(mouse) => visible_events.push((now, format!("{mouse:?}"))),
                Event::Resize(width, height) => {
                    visible_events.push((now, format!("Resize({width}, {height})")))
                }
                _ => {}
            }
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
                    visible_events
                        .iter()
                        .rev()
                        .take(f.area().height as usize - 2)
                        .map(|(time, event)| {
                            Line::from(format!(
                                "{:>14} ago: {:?}",
                                humantime::format_duration(Duration::from_millis(
                                    time.elapsed().as_millis() as u64
                                ))
                                .to_string(),
                                event
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
    execute!(stdout(), DisableMouseCapture).unwrap();
    ratatui::restore();
}
