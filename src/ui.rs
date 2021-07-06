use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;
use tui::layout::{Layout, Direction, Constraint};
use tui::text::{Span, Spans};
use tui::style::{Style, Modifier, Color};
use tui::widgets::{Tabs, Block, Borders};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>
}

pub fn new() -> Ui {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    let _ = terminal.clear();
    Ui{
        terminal
    }
}

impl Ui {
    pub fn render(&mut self) {
        let _ = self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0)
                ])
                .split(f.size());
            let titles = vec![
                Spans::from(vec![
                    Span::styled("first", Style::default()),
                ]),
                Spans::from(vec![
                    Span::styled("rest", Style::default()),
                ])
            ];
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Passman"))
                .select(0)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Green),
                );
            f.render_widget(tabs, chunks[0]);
            let block = Block::default()
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        });
    }
}
