use serenity::client::{Context, CACHE};
use serenity::model::{Message, GuildId, ChannelId};
use preferences::{PreferencesMap, Preferences};
use std::str::FromStr;

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

pub fn owner_check(_: &mut Context, message: &Message) -> bool {
    if let Ok(owner_pref) = PreferencesMap::<String>::load(&::APP_INFO, ::PREF_OWNER_KEY) {
        if let Some(owner_key) = owner_pref.get("owner_id") {
            if let Ok(pref_id) = u64::from_str(owner_key) {
                message.author.id == pref_id
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn get_user_audio_channel(message: &Message) -> Option<(GuildId, Option<ChannelId>)> {
    let guild_id = ron!(message.guild_id());
    let cache_lock = roe!(CACHE.try_read());
    let guild_lock = ron!(cache_lock.get_guild(guild_id));
    let guild = roe!(guild_lock.try_read());

    guild.voice_states.get(&message.author.id).map(|x| (guild_id, x.channel_id))
}

// TODO: implement a check for disabled commands through user preferences
pub fn enabled_check(_: &mut Context, message: &Message) -> bool {
    return true;
}
