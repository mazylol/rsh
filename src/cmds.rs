use std::collections::HashMap;

use crate::cmd_hashmap;

pub fn builtins() -> HashMap<&'static str, fn(&Vec<&str>)> {
    let builtin_commands: HashMap<&str, fn(&Vec<&str>)> = cmd_hashmap!("cd" => |args| {
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
    }, "exit" => |args| {
        if args.len() == 1 {
            std::process::exit(0);
        }

        std::process::exit(args[1].parse().unwrap());
    }, "?" => |_| {
        println!("This is a simple shell written in Rust");
    }, "clear" => |_| {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    });

    builtin_commands
}
