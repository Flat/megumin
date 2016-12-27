// Owner commands
// These are only usable by the bot's owner


use preferences::{PreferencesMap, Preferences};
use serenity::model::Game;

// Sets the owner to the calling user if an owner is not already set
command!(set_owner(_context, message) {
    if let Ok(mut owner_pref) = PreferencesMap::<String>::load(&::APP_INFO, ::PREF_OWNER_KEY) {
        if !owner_pref.contains_key("owner_id"){
            owner_pref.insert("owner_id".into(), message.author.id.to_string());
            if let Err(why) = owner_pref.save(&::APP_INFO, ::PREF_OWNER_KEY) {
                println!("Failed to set owner: {:?}", why);
            } else {
                let _ = message.reply("Set you as the owner!");
            }
        }
    } else {
        let mut owner_pref: PreferencesMap<String> = PreferencesMap::new();
        owner_pref.insert("owner_id".into(), message.author.id.to_string());
        if let Err(why) = owner_pref.save(&::APP_INFO, ::PREF_OWNER_KEY){
            println!("Failed to set owner: {:?}", why);
        } else {
            let _ = message.reply("Set you as the owner!");
        }
    }
});

// tells you the userID of the bot owner
command!(view_owner(_context, message) {
    if let Ok(owner_pref) = PreferencesMap::<String>::load(&::APP_INFO, ::PREF_OWNER_KEY){
        if let Some(owner_id) = owner_pref.get("owner_id") {
            let _ = message.reply(&owner_id.to_string());
        } else {
            let _ = message.reply("null");
        }
    } else {
        let _ = message.reply("null");
    }
});

// Removes the owner from the bot. Can only be used by the current owner
command!(disown(_context, message) {
    if let Ok(mut owner_pref) = PreferencesMap::<String>::load(&::APP_INFO, ::PREF_OWNER_KEY){
        if owner_pref.contains_key("owner_id") {
            owner_pref.remove("owner_id");
            if let Err(why) = owner_pref.save(&::APP_INFO, ::PREF_OWNER_KEY){
                println!("Error removing owner: {:?}", why);
            } else {
                let _ = message.reply("Owner has been removed.");
            }
        }
    }
});

// Sets the now playing text
command!(set_game(context, _message, args) {
    let game_name = args.join(" ");
    context.set_game(Game::playing(&game_name));
});

// Clears the now playing text
command!(no_game(context, _message) {
    context.reset_presence();
});
