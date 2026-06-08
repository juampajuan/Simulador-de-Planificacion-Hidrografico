pub mod requests;
mod interaction; 
use crate::interaction::print;
use crate::interaction::logic;

fn main() {
    print::print_banner();
    print::print_help();

    // Ver como consigo esto.
    let host = format!("http://localhost:{}", 3000);
    loop {
        logic::menu(&host);
    }
}