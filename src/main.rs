use eframe::{NativeOptions, run_native};

use app::Passman;

mod app;
mod page;
mod state;
mod storage;

fn main() -> eframe::Result<()> {
    run_native(
        "passman",
        NativeOptions::default(),
        Box::new(|ctx| Box::new(Passman::new(ctx)))
    )
}
