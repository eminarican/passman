use crossterm::event::KeyCode;

mod input;
mod ui;

struct Passman {
    input: input::Input,
    ui: ui::Ui,
}

impl Passman {
    fn new() -> Passman {
        Passman {
            input: input::new(),
            ui: ui::new()
        }
    }

    fn start(self) {
        loop {
            self.ui.render();
            if let Some(key) = self.input.handle() {
                match key {
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        println!("bye!")
    }
}

fn main() {
    let passman = Passman::new();
    passman.start()
}
