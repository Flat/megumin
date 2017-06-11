// Owner commands
// These are only usable by the bot's owner

use serenity::model::Game;
use util::preferences;
use util::settings::*;

// Sets the owner to the calling user if an owner is not already set
command!(set_owner(_context, message) {
    match preferences::load_owner_config() {
        Ok(_) => (),
        Err(_) => {
            let new_conf = OwnerSettings { owner_id: message.author.id.0, audio_max_length: None, audio_cache_max: None };
            match preferences::save_owner_config(new_conf) {
                Ok(_) => {let _ = message.reply("set you as the owner!");},
                Err(why) => {let _ = message.reply(&why.to_string());}, 
            };
        },
    };
});

// tells you the userID of the bot owner
command!(view_owner(_context, message) {
    if let Ok(config) = preferences::load_owner_config() {
        let _ = message.reply(&config.owner_id.to_string());
    } else {
        let _ = message.reply("null");
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
