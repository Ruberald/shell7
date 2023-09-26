use std::io::{Error, self, Write};

fn main() -> Result<(), Error> {
    let mut command = String::new();

    let shell_prompt = "--7";
    println!("Welcome to shell7!");

    loop {
        print!("{shell_prompt} ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut command)?;
        print!("You entered: {}", command);
        command.clear();
    }
}
