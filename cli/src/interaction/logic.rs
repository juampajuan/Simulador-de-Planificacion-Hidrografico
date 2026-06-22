use reqwest::blocking::Client;
use crate::requests;
use super::print;
use std::env;

/// Realiza el manejo logico del menu
// Toma un input y ejecuta el metodo que corresponde.
pub fn menu(host:&str, client: &Client) {

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
            match requests::create_user(client, host, user, pass) {
                Ok(response) => println!("{}", response),
                Err(err) => eprintln!("\x1b[31mError:\x1b[37m {}\x1b[0m", err)
            }
        }

        ["newpass", user, pass] => {
            match requests::change_pass(client, host, user, pass) {
                Ok(response) => println!("{}", response),
                Err(err) => eprintln!("\x1b[31mError:\x1b[37m {}\x1b[0m", err)
            }
        }

        ["closeall"] => {
            match requests::close_all(client, host) {
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

/// Recibe y procesa los argumentos que recibe al ejecutarlo standalone 
pub fn get_args() -> Option<(String, String)> {
    let mut args = env::args().skip(1);

    let host = args.next()?;
    let password = args.next()?;
    Some((format_host(&host), password))
}

/// Agrega formato a la url introducida como args
// Sin el http. El crate no la puede usar.
fn format_host(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("http://{}", url)
    }
}