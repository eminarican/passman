use app::Subcommand;

mod storage;
mod app;

fn main() {
    let matches = app::new();
    let storage = storage::new(&matches);

    match app::subcommand(&matches) {
        Subcommand::Set{ provider, value } => {

        }
        Subcommand::Get{ provider } => {

        }
        Subcommand::Gen{ provider } => {

        }
        Subcommand::Del{ provider } => {

        }
    }
}
