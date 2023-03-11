use eframe::{App, CreationContext, Frame};
use egui::{Context, Ui};

use std::process::exit;
use copypasta::{ClipboardContext, ClipboardProvider};

use super::{
    state::{State, Message},
    storage::Storage,
    page::Page,
};

#[derive(Default)]
pub struct Passman {
    page: Page,
    state: State,
}

impl Passman {
    pub fn login(&mut self, password: String) -> bool {
        if password.is_empty() {
            return false
        }

        let storage = Storage::new(password);
        if storage.is_none() {
            self.state.attempt -= 1;

            if self.state.attempt == 0 {
                exit(0)
            }
            return false
        }

        self.state.storage = storage.unwrap();
        true
    }

    pub fn remove(&mut self, provider: String) -> bool {
        if provider.is_empty() {
            return false
        }

        self.state.storage.del(provider)
    }

    pub fn copy(&mut self, provider: String) -> bool {
        if provider.is_empty() {
            return false
        }

        if let Ok(mut ctx) = ClipboardContext::new() {
            ctx.set_contents(provider).is_ok()
        } else {
            false
        }
    }

    pub fn generate(&mut self, provider: String) -> bool {
        if provider.is_empty() {
            return false
        }

        self.state.storage.gen(provider)
    }

    pub fn add(&mut self, provider: String, password: String) -> bool {
        if provider.is_empty() || password.is_empty() {
            return false
        }

        self.state.storage.set(provider, password)
    }
}

impl App for Passman {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("navigation").show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.heading("🔐 passman");

                if self.page == Page::Login {
                    return;
                }

                ui.add_space(5.0);
                if ui.button("🏠 list all").clicked() {
                    self.page = Page::List
                }
                if ui.button("➕ add new").clicked() {
                    self.page = Page::Add
                }
                if ui.button("❓ generate").clicked() {
                    self.page = Page::Generate
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_page(ctx, ui)
        });
    }

    fn on_close_event(&mut self) -> bool {
        self.state.storage.save()
    }
}

impl Passman {
    pub fn new(_ctx: &CreationContext) -> Self {
        Self::default()
    }

    pub fn update_page(&mut self, ctx: &Context, ui: &mut Ui) {
        if match self.page.render(ctx, ui, &mut self.state) {
            Message::Login {
                password
            } => self.login(password),
            Message::Remove {
                provider
            } => self.remove(provider),
            Message::Copy {
                provider
            } => self.copy(provider),
            Message::Generate {
                provider
            } => self.generate(provider),
            Message::Add {
                provider,
                password,
            } => self.add(provider, password),
            Message::None => false
        } {
            self.state.clear_fields();
            self.page = Page::List
        }
    }
}
