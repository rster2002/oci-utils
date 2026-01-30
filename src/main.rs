use crate::modules::app;

mod modules;

fn main() {
    match app::run() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }
}
