use app_dirs;
use serenity::client::{Context, CACHE};
use serenity::model::{Message, GuildId, ChannelId};
use serde_json;
use std::fmt::{self, Display};
use std::io;

pub mod preferences;
pub mod settings;

// Return on Err
macro_rules! roe {
    ($x:expr) => {{
        match $x {
            Ok(v) => v,
            Err(_) => return None,
        }
    }}
}

// Return on None
macro_rules! ron {
    ($x:expr) => {{
        match $x {
            Some(v) => v,
            None => return None,
        }
    }}
}

pub enum Error {
    AppDirError(app_dirs::AppDirsError),
    FileError(io::Error),
    JsonError(serde_json::Error),
}

impl From<app_dirs::AppDirsError> for Error{
    fn from(e: app_dirs::AppDirsError) -> Error{
        Error::AppDirError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::FileError(e)
    }
}

impl From<serde_json::Error> for Error  {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AppDirError(ref inner) => inner.fmt(f),
            Error::FileError(ref inner) => inner.fmt(f),
            Error::JsonError(ref inner) => inner.fmt(f),
        }
    }
}

pub fn owner_check(_: &mut Context, message: &Message) -> bool {
    if let Ok(config) = preferences::load_owner_config() {
        message.author.id.0 == config.owner_id
    } else {
        false
    }
}

pub fn get_user_audio_channel(message: &Message) -> Option<(GuildId, Option<ChannelId>)> {
    let guild_id = ron!(message.guild_id());
    let cache_lock = roe!(CACHE.try_read());
    let guild_lock = ron!(cache_lock.guild(guild_id));
    let guild = roe!(guild_lock.try_read());

    guild.voice_states.get(&message.author.id).map(|x| (guild_id, x.channel_id))
}

// TODO: implement a check for disabled commands through user preferences
pub fn enabled_check(_: &mut Context, message: &Message) -> bool {
    let guild_id = match message.guild_id() {
        Some(i) => i,
        None => return true,
    };

    if let Ok(config) = preferences::load_guild_config(guild_id.0) {
        true
    } else {
        true
    }
}
