use crate::requests;
use super::print;

pub fn menu(host:&str) {

    let input = match print::input("> ") {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Error leyendo input: {}", err);
            return;
        }
    };

    let args: Vec<&str> = input.split_whitespace().collect();

    match args.as_slice() {
        ["create", user, pass] => {
            match requests::create_user(host, user, pass) {
                Ok(response) => println!("{}", response),
                Err(err) => eprintln!("\x1b[31mError:\x1b[37m {}\x1b[0m", err)
            }
        }

        ["newpass", user, pass] => {
            match requests::change_pass(host, user, pass) {
                Ok(response) => println!("{}", response),
                Err(err) => eprintln!("\x1b[31mError:\x1b[37m {}\x1b[0m", err)
            }
        }

        ["help"] | ["h"] | ["HELP"] | ["H"] => {
            print::print_help();
        }

        _ => {
            print::unknown_command();
        }
    }
    
}