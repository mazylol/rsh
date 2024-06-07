mod cmds;

use colored::Colorize;

use std::io::{self, Write};

use whoami::fallible;

fn main() -> anyhow::Result<()> {
    let builtin_commands = cmds::builtins();

    loop {
        print!(
            "{}@{}:{}$ ",
            whoami::username().green().bold(),
            fallible::hostname()?.green().bold(),
            std::env::current_dir().unwrap().display().to_string().blue().bold()
        );
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let splitted: Vec<&str> = input.split_whitespace().collect();

        if splitted.len() == 0 {
            continue;
        }

        let command = splitted[0];
        let args = &splitted[1..];

        if builtin_commands.contains_key(command) {
            let command_fn = builtin_commands.get(command).unwrap();
            command_fn(&splitted);
        } else {
            let child = std::process::Command::new(command).args(args).spawn();

            match child {
                Ok(mut child) => {
                    child.wait().expect("failed to wait on child");
                }
                Err(_) => {
                    eprintln!("Command '{}' not found", command);
                }
            }
        }
    }
}
