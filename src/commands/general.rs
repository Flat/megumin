use chrono::*;
use serenity::CACHE;
use serenity::model::permissions::*;
use serenity::utils::Colour;
use Uptime;

// Displays various information about this bot software
command!(about(_context, msg, _args) {
  let cache = match CACHE.read() {
      Ok(cache) => cache,
      Err(why) => {
      println!("Failed to read cache: {:?}", why);
      return Err(why.to_string());
    },
  };
  let _ = msg.channel_id.send_message(|m| m
      .embed(|e| e
        .url("https://github.com/flat/megumin")
        .colour(Colour::new(0xC4444E))
        .description("A general purpose discord bot built with the serenity library. \
         Programmed in Rust and made with love.")
        .title("Megumin")
        .author(|mut a| {
          a = a.name(&::BOT_NAME);
          // Bot avatar URL
          a = a.icon_url(&cache.user.face());
          a
        })
        .field(|f| f
          .name("Authors")
          // Author's const from cargo package authors field
          .value(&::AUTHORS)
          )
        )
  );
});

// Displays information about the current bot instance
command!(info(context, msg, _args) {
  let cache = match CACHE.read() {
    Ok(cache) => cache,
    Err(why) => {
      println!("Failed to read cache: {:?}", why);
      return Err(why.to_string());
    },
  };

  // Get startup time from context.data
  let data = context.data.lock().unwrap();
  let uptime = data.get::<Uptime>().unwrap();

  if let Some(boottime) = uptime.get("boot") {
    let now = UTC::now();
    let duration = now.signed_duration_since(boottime.to_owned());
    // Transform duration into days, hours, minutes, seconds.
    // There's probably a cleaner way to do this.
    let mut seconds = duration.num_seconds();
    let mut minutes = seconds / 60;
    seconds %= 60;
    let mut hours = minutes / 60;
    minutes %= 60;
    let days = hours / 24;
    hours %= 24;

    let invite_url = match cache.user.invite_url(READ_MESSAGES | SEND_MESSAGES | EMBED_LINKS | ADD_REACTIONS | READ_MESSAGE_HISTORY | USE_EXTERNAL_EMOJIS | CONNECT | USE_VAD | CHANGE_NICKNAME) {
      Ok(s) => s,
      Err(why) => {
        println!("Failed to get invite url: {:?}", why);
        return Err(why.to_string());
      } 
    };

    let _ = msg.channel_id.send_message(|m| m
      .embed(|e| e
        .url(&invite_url)
        .colour(Colour::fabled_pink())
        .description(&format!("I'm currently running {} - {}", &::BOT_NAME, &::VERSION))
        .title("Invite me to your server!")
        .author(|mut a| {
          a = a.name(&cache.user.name);
          a = a.icon_url(&cache.user.face());
          a
        })
        .field(|f| f
          .name("Uptime")
          .value(&format!("{}d{}h{}m{}s", days, hours , minutes, seconds))
          )
        .field(|f| f
          .name("Guilds")
          .value(&cache.guilds.len().to_string())
          )
        .field(|f| f
          .name("Private Channels")
          .value(&cache.private_channels.len().to_string())
          )
        )
      );
  }
  // If we can't read the context.data give up
  else {
    let _ = msg.channel_id.say("Unable to get startup time");
  }

});
