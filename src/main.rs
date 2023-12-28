use std::process::Command;
use std::io::{self, Write};

fn handle_command(command: &String) -> Result<&str, ()> {
    print!("You entered: {}", command);

    match command {
        _ => Err(())
    }
}

fn main() -> io::Result<()> {

    let mut command = String::new();

    let shell_prompt = "--7";
    println!("Welcome to shell7!");

    loop {
        print!("{shell_prompt} ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut command)?;

        if command == "exit\n" {
            break;
        }

        let output = Command::new(&command.trim())
            .stdout(io::stdout())
            .spawn()?
            .wait_with_output()?;

        if !output.status.success() {
            eprintln!("Command failed!");
        }

        match handle_command(&command) {
            Ok(_) => (),
            Err(()) => eprintln!("Command doesn't exist"),
        }

        command.clear();
    }

    Ok(())

}
