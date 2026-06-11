pub mod requests;
mod interaction; 
use crate::interaction::print;
use crate::interaction::logic;

fn main() {

    let (host, pass) = match logic::get_args() {
        Some(args) => args,
        None => {
            eprintln!("Ejecutar como: \x1b[36m./programa\x1b[0m \x1b[95m<host>\x1b[0m \x1b[95m<password>\x1b[0m");
            return;
        }
    };

    print::print_banner();
    print::print_help();
    loop {
        logic::menu(&host, &pass);
    }
}