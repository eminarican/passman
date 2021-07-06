use app::Subcommand;

mod storage;
mod app;

fn main() {
    let matches = app::new();
    let storage = storage::new(&matches);

    match app::subcommand(&matches) {
        Subcommand::Set => {

        }
        Subcommand::Get => {

        }
        Subcommand::Gen => {

        }
        Subcommand::Del => {

        }
    }
}
