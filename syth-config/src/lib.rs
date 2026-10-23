use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct Config {
    pub minecraft: MinecraftConfig,
    //pub classic_extension: CPEConfig,
    pub misc: MiscellaneousConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct MinecraftConfig {
    verify_names: bool,
    admin_slot: bool,
    port: u16,
    public: bool,
    server_name: String,
    max_players: u8,
    max_connections: u8,
    motd: String,
    grow_trees: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct MiscellaneousConfig {
    generator: String,
}

impl Default for MinecraftConfig {
    fn default() -> Self {
        Self {
            verify_names: true,
            admin_slot: true,
            port: 25565,
            public: true,
            server_name: String::from("Syth Server"),
            max_players: 16,
            max_connections: 3,
            motd: String::from("A classic Minecraft Syth server."),
            grow_trees: true,
        }
    }
}

impl Default for MiscellaneousConfig {
    fn default() -> Self {
        Self {
            generator: String::from("FLAT"),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config = Self::load_from(Path::new("config.toml"));
        config
    }

    pub fn load_from(path: &Path) -> Self {
        let Ok(content) = fs::read_to_string(path) else {
            println!("Config: not found, creating new one");

            let config = Config::default();

            fs::write(path, toml::to_string_pretty(&config).unwrap())
                .expect("Config: failed to write to config file.");
            return config;
        };

        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}
