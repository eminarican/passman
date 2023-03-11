use egui::{Context, RichText, Ui};

use crate::{
    state::{State, Message},
    page,
};

#[derive(Eq, PartialEq)]
pub enum Page {
    Login,
    List,
    Add,
    Generate,
}

impl Page {
    pub fn render(&self, ctx: &Context, ui: &mut Ui, state: &mut State) -> Message {
        match self {
            Page::Login => login(ctx, ui, state),
            Page::List => list(ctx, ui, state),
            Page::Add => add(ctx, ui, state),
            Page::Generate => generate(ctx, ui, state),
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::Login
    }
}

page!(login, |_ctx: &Context, ui: &mut Ui, state: &mut State, message: &mut Message| {
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() * 0.25);
        ui.heading(RichText::new("please enter your password").strong());
        ui.label(format!("remaining attempt: {}", state.attempt));
        ui.add_space(ui.available_height() * 0.25);
        ui.add_space(5.0);
        ui.add(egui::TextEdit::singleline(&mut state.password).hint_text("password").password(!state.show));
        ui.checkbox(&mut state.show, "show password");
        ui.add_space(5.0);
        if ui.button("login").clicked() {
            *message = Message::Login {
                password: state.password.clone()
            }
        }
    });
});

page!(list, |_ctx: &Context, ui: &mut Ui, state: &mut State, message: &mut Message| {
    if state.storage.len() == 0 {
        ui.label("there's no password");
        return
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(ui.available_width());

        for (provider, password) in state.storage.clone() {
            ui.horizontal(|ui| {
                if ui.button("🗑 remove").clicked() {
                    *message = Message::Remove {
                        provider: provider.clone()
                    }
                }
                if ui.button("📄 copy").clicked() {
                    *message = Message::Copy {
                        provider: password.clone()
                    }
                }
                ui.heading(RichText::new(provider).strong());
                ui.label(password);
            });
        }
    });
});

page!(add, |_ctx: &Context, ui: &mut Ui, state: &mut State, message: &mut Message| {
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() * 0.25);
        ui.heading(RichText::new("please enter a provider and password to add").strong());
        ui.add_space(ui.available_height() * 0.25);
        ui.add(egui::TextEdit::singleline(&mut state.provider).hint_text("provider"));
        ui.add_space(5.0);
        ui.add(egui::TextEdit::singleline(&mut state.password).hint_text("password").password(!state.show));
        ui.checkbox(&mut state.show, "show password");
        ui.add_space(5.0);
        if ui.button("add").clicked() {
            *message = Message::Add {
                provider: state.provider.clone(),
                password: state.password.clone(),
            }
        }
    });
});

page!(generate, |_ctx: &Context, ui: &mut Ui, state: &mut State, message: &mut Message| {
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() * 0.25);
        ui.heading(RichText::new("please enter a provider to generate password").strong());
        ui.add_space(ui.available_height() * 0.25);
        ui.add(egui::TextEdit::singleline(&mut state.provider).hint_text("provider"));
        ui.add_space(5.0);
        if ui.button("generate").clicked() {
            *message = Message::Generate {
                provider: state.provider.clone()
            }
        }
    });
});

#[macro_export]
macro_rules! page {
    ($name:ident, $body:expr $(,)?) => {
        fn $name(ctx: &Context, ui: &mut Ui, state: &mut State) -> Message {
            let mut message = Message::None;
            $body(ctx, ui, state, &mut message);
            message
        }
    };
}
