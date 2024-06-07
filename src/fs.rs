use crate::{config::Config, hashmap};

pub fn load_config_str() -> String {
    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.join("rsh");
    let config_file = config_dir.join("config.ron");

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).unwrap();
    }

    if !config_file.exists() {
        let default_config = Config {
            aliases: hashmap!("ll" => "ls -l", "la" => "ls -a"),
            paths: vec![],
            env_vars: hashmap!("EDITOR" => "vim"),
        };

        let default_config =
            ron::ser::to_string_pretty(&default_config, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write(&config_file, &default_config).unwrap();

        return default_config.to_string();
    } else {
        let contents = std::fs::read_to_string(&config_file).unwrap();
        return contents;
    }
}
