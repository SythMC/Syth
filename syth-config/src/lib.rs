use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, path::Path};

// TODO: If anything gets added make sure the config gets updated
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct Config {
    pub server: MinecraftConfig,
    pub cpe: CPEConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct MinecraftConfig {
    pub server_name: String,
    pub motd: String,
    pub ip: String,
    pub port: u16,
    pub max_players: u32,
    pub public: bool,
    pub verify_names: bool,
    pub max_connections: u32,
    pub grow_trees: bool,
    pub admin_slot: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct CPEConfig {
    pub hack_control_flying: u8,
    pub hack_control_noclip: u8,
    pub hack_control_speeding: u8,
    pub hack_control_spawn_control: u8,
    pub hack_control_third_person_view: u8,
    pub hack_control_jump_height: i16,
}

impl Default for MinecraftConfig {
    fn default() -> Self {
        Self {
            server_name: "Minecraft Server".to_string(),
            motd: "Welcome to my Rust Minecraft Server!".to_string(),
            ip: "0.0.0.0".to_string(),
            port: 25565,
            max_players: 16,
            public: true,
            verify_names: true,
            max_connections: 3,
            grow_trees: false,
            admin_slot: false,
        }
    }
}

impl Default for CPEConfig {
    fn default() -> Self {
        Self {
            hack_control_flying: 1,
            hack_control_noclip: 1,
            hack_control_speeding: 1,
            hack_control_spawn_control: 1,
            hack_control_third_person_view: 1,
            hack_control_jump_height: 40,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::load_from(&data_dir().join("config.toml"))
    }

    pub fn load_from(path: &Path) -> Self {
        if !path.exists() {
            let config = Config::default();

            fs::write(path, toml::to_string_pretty(&config).unwrap()).unwrap();

            return config;
        }

        let contents = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();

        config
    }
}

// AI start
pub fn data_dir() -> PathBuf {
    let dir = match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest) => PathBuf::from(manifest)
            .parent()
            .expect("manifest dir has a parent")
            .join("svr"),
        None => std::env::current_exe()
            .expect("can't find exe path")
            .parent()
            .expect("exe has a parent")
            .to_path_buf(),
    };
    std::fs::create_dir_all(&dir).ok();
    dir
}
// AI end
