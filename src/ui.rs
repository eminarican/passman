use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>
}

pub fn new() -> Ui {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    terminal.clear();
    Ui{
        terminal
    }
}

impl Ui {
    pub fn render(&self) {
    }
}
