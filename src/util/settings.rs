#[derive(Serialize, Deserialize, Debug)]
pub struct GuildSettings {
    pub guild_id: u64,
    pub disabled_commands: Option<Vec<String>>,
    pub ignored_users: Option<Vec<u64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnerSettings {
    pub owner_id: u64,
    pub audio_cache_max: Option<u32>,
    pub audio_max_length: Option<u64>,
}