use std::collections::HashMap;

pub fn builtins() -> HashMap<&'static str, fn(&Vec<&str>)> {
    let mut builtin_commands: HashMap<&str, fn(&Vec<&str>)> = HashMap::new();

    builtin_commands.insert("cd", |args| {
        if args.len() == 1 {
            let home = std::env::var("HOME").unwrap();
            if let Err(e) = std::env::set_current_dir(home) {
                println!("cd: {}", e);
            }
        } else {
            let path = args[1];
            if let Err(e) = std::env::set_current_dir(path) {
                println!("cd: {}", e);
            }
        }
    });

    builtin_commands.insert("exit", |args| {
        if args.len() == 1 {
                std::process::exit(0);
        }

        std::process::exit(args[1].parse().unwrap());
    });

    builtin_commands.insert("?", |_| {
        println!("This is a simple shell written in Rust");
    });

    builtin_commands
}