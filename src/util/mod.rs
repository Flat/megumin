use serenity::client::Context;
use serenity::model::Message;
use preferences::{PreferencesMap, Preferences};
use std::str::FromStr;


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
