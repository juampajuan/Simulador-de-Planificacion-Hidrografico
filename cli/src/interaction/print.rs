use std::io::{self, Write};

const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

// Imprime el banner
// SOLO mostrado si se ejecuta standalone
pub fn print_banner() {
    let space = 32 as char;

    println!(
        "{BLUE}\
        {space}___ ___ __  __ _   _ _      _   ___   ___  ___   _  _ ___ ___  ___  ___   ___ ___    _   ___ ___ ___ ___     ___ _    ___ \n\
        / __|_ _|  \\/  | | | | |    /_\\ |   \\ / _ \\| _ \\ | || |_ _|   \\| _ \\/ _ \\ / __| _ \\  /_\\ | __|_ _/ __/ _ \\   / __| |  |_ _|\n\
        \\__ \\| || |\\/| | |_| | |__ / _ \\| |) | (_) |   / | __ || || |) |   / (_) | (_ |   / / _ \\| _| | | (_| (_) | | (__| |__ | | \n\
        |___/___|_|  |_|\\___/|____/_/ \\_\\___/ \\___/|_|_\\ |_||_|___|___/|_|_\\\\___/ \\___|_|_\\/_/ \\_\\_| |___\\___\\___/   \\___|____|___|\n\
        {RESET}"
    );
    println!("Bienvenido al CLI del simulador. Este permite añadir docentes y restaurar contraseñas.");
}

// Mensaje para comando desconocido
pub fn unknown_command() {
    println!(
        "{BLUE}Comando desconocido.{RESET} Escriba \
        {CYAN}H{RESET} o \
        {CYAN}HELP{RESET} \
        para ayuda."
    );
}

// Imprime una guia/ayuda memoria de los metodos disponibles.
pub fn print_help() {
    let commands = vec![
        (
            "Crear un nuevo usuario.",
            "create",
            "<username> <pass>"
        ),
        (
            "Cambiar contraseña de usuario.",
            "newpass",
            "<username> <newpass>"
        ),
        (
            "Cerrar todas las sesiones.",
            "closeall",
            ""
        ),
    ];

    println!();
    for (description, command, params) in commands {
        print_command(description, command, params);
    }
}

// Metodo generico, para imprimir cada comando
fn print_command(description: &str, command: &str, params: &str) {
    println!(
        "{BLUE}{}{RESET}\n\
         └─ {CYAN}{}{RESET} {MAGENTA}{}{RESET}\n",
        description,
        command,
        params
    );
}

// Toma el input del usuario y lo procesa.
pub fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().into())
}