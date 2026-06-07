use std::io::{self, Write};

pub fn print_banner() {
    let space = 32 as char;

    println!(
        "\x1b[34m\
        {space}___ ___ __  __ _   _ _      _   ___   ___  ___   _  _ ___ ___  ___  ___   ___ ___    _   ___ ___ ___ ___     ___ _    ___ \n\
        / __|_ _|  \\/  | | | | |    /_\\ |   \\ / _ \\| _ \\ | || |_ _|   \\| _ \\/ _ \\ / __| _ \\  /_\\ | __|_ _/ __/ _ \\   / __| |  |_ _|\n\
        \\__ \\| || |\\/| | |_| | |__ / _ \\| |) | (_) |   / | __ || || |) |   / (_) | (_ |   / / _ \\| _| | | (_| (_) | | (__| |__ | | \n\
        |___/___|_|  |_|\\___/|____/_/ \\_\\___/ \\___/|_|_\\ |_||_|___|___/|_|_\\\\___/ \\___|_|_\\/_/ \\_\\_| |___\\___\\___/   \\___|____|___|\n\
        \x1b[0m"
    );
    println!("Bienvenido al CLI del simulador. Este permite añadir docentes y restaurar contraseñas.");
}

pub fn unknown_command() {
    println!(
        "\x1b[34mComando desconocido.\x1b[0m Escriba \
        \x1b[36mH\x1b[0m o \
        \x1b[36mHELP\x1b[0m \
        para ayuda."
    );
}

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
    ];

    print!("\n");
    for (description, command, params) in commands {
        print_command(description, command, params);
    }
}

fn print_command(description: &str, command: &str, params: &str) {
    println!(
        "\x1b[34m{}\x1b[0m\n\
         └─ \x1b[36m{}\x1b[0m \x1b[35m{}\x1b[0m\n",
        description,
        command,
        params
    );
}

pub fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().into())
}