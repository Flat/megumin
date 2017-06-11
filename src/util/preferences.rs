use app_dirs::*;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use util::Error as UtilError;
use util::settings::*;

pub fn load_owner_config() -> Result<OwnerSettings, UtilError> {
    let mut config_dir = app_root(AppDataType::UserConfig, &::APP_INFO)?;
    config_dir.push("settings.json");
    let mut file = File::open(config_dir)?;
    let mut config = String::new();
    file.read_to_string(&mut config)?;
    
    let confobj: OwnerSettings = serde_json::from_str(&config)?;
    Ok(confobj)
}

pub fn save_owner_config(config: OwnerSettings) -> Result<(), UtilError> {
    let mut config_dir = app_root(AppDataType::UserConfig, &::APP_INFO)?;
    config_dir.push("settings.json");
    let mut file = File::create(config_dir)?;
    file.write_all(&serde_json::to_string(&config)?.as_bytes())?;
    Ok(())
}

pub fn load_guild_config(guild_id: u64) -> Result<GuildSettings, UtilError> {
    let mut config_dir = app_root(AppDataType::UserConfig, &::APP_INFO)?;
    config_dir.push(guild_id.to_string());
    config_dir.push("settings");
    config_dir.set_extension("json");
    let mut file = File::open(config_dir)?;
    let mut config = String::new();
    file.read_to_string(&mut config)?;
    
    let confobj: GuildSettings = serde_json::from_str(&config)?;
    Ok(confobj)
}

pub fn save_guild_config(config: GuildSettings) -> Result<(), UtilError> {
    let mut config_dir = app_root(AppDataType::UserConfig, &::APP_INFO)?;
    config_dir.push(config.guild_id.to_string());
    config_dir.push("settings");
    config_dir.set_extension("json");
    let mut file = File::create(config_dir)?;
    file.write_all(&serde_json::to_string(&config)?.as_bytes())?;
    Ok(())
}