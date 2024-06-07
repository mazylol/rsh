mod cmds;
mod config;
mod fs;
mod macros;

use colored::Colorize;

use std::io::{self, Write};

use whoami::fallible;

use ctrlc::set_handler;
use std::process::{exit, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let config_str = fs::load_config_str();
    let config = config::Config::from_ron(&config_str);

    for (key, value) in config.env_vars {
        std::env::set_var(key, value);
    }

    let builtin_commands = cmds::builtins();

    loop {
        print!(
            "{}@{}:{}$ ",
            whoami::username().green().bold(),
            fallible::hostname()?.green().bold(),
            std::env::current_dir()
                .unwrap()
                .display()
                .to_string()
                .blue()
                .bold()
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

        if config.aliases.contains_key(command) {
            let alias = config.aliases.get(command).unwrap();
            let child = std::process::Command::new("sh")
                .arg("-c")
                .arg(alias)
                .spawn();

            match child {
                Ok(mut child) => loop {
                    match child.try_wait() {
                        Ok(Some(_status)) => break,
                        Ok(None) => {
                            if !running.load(Ordering::SeqCst) {
                                child.kill().expect("command wasn't running");
                                child.wait().expect("failed to wait on child");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to wait on child: {}", e);
                            exit(1);
                        }
                    }
                },
                Err(_) => {
                    eprintln!("Alias '{}' not found", command);
                }
            }

            continue;
        }

        if builtin_commands.contains_key(command) {
            let command_fn = builtin_commands.get(command).unwrap();
            command_fn(&splitted);

            continue;
        } else {
            let child = Command::new(command).args(args).spawn();

            match child {
                Ok(mut child) => loop {
                    match child.try_wait() {
                        Ok(Some(_status)) => break,
                        Ok(None) => {
                            if !running.load(Ordering::SeqCst) {
                                child.kill().expect("command wasn't running");
                                child.wait().expect("failed to wait on child");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to wait on child: {}", e);
                            exit(1);
                        }
                    }
                },
                Err(_) => {
                    let mut command_found = false;
                    for path in &config.paths {
                        let path = std::path::Path::new(path);
                        let path = path.join(command);

                        if path.exists() {
                            let child = std::process::Command::new(path).args(args).spawn();

                            match child {
                                Ok(mut child) => {
                                    loop {
                                        match child.try_wait() {
                                            Ok(Some(_status)) => break,
                                            Ok(None) => {
                                                if !running.load(Ordering::SeqCst) {
                                                    child.kill().expect("command wasn't running");
                                                    child.wait().expect("failed to wait on child");
                                                    break;
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to wait on child: {}", e);
                                                exit(1);
                                            }
                                        }
                                    }

                                    command_found = true;
                                }
                                Err(_) => {
                                    eprintln!("Command '{}' not found", command);
                                }
                            }

                            break;
                        }
                    }
                    if !command_found {
                        eprintln!("Command '{}' not found", command);
                    }
                }
            }
        }

        continue;
    }
}
