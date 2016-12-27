use serenity::client::CACHE;
use serenity::utils::Colour;
use chrono::*;
use Uptime;

// Displays various information about this bot software
command!(about(context) {
  let _ = context.send_message(|m| m
      .embed(|e| e
        .url("https://github.com/flat/megumin")
        // TODO: Change colour to Megumin's red robe color
        .colour(Colour::fabled_pink())
        .description("A general purpose discord bot built with the serenity library. \
         Programmed in Rust and made with love.")
        .title("Megumin")
        .author(|mut a| {
          a = a.name(&::BOT_NAME);
          // Bot avatar URL
          a = a.icon_url("https://files.catbox.moe/r8r2h1.png");
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
command!(info(context) {
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
    let duration = now - boottime.to_owned();
    // Transform duration into days, hours, minutes, seconds.
    // There's probably a cleaner way to do this.
    let mut seconds = duration.num_seconds();
    let mut minutes = seconds / 60;
    seconds = seconds % 60;
    let mut hours = minutes / 60;
    minutes = minutes % 60;
    let days = hours / 24;
    hours = hours % 24;


    let _ = context.send_message(|m| m
      .embed(|e| e
        .url(&format!(
          // Link to invite bot through discord api using bot's id
          "https://discordapp.com/api/oauth2/authorize?client_id={}&scope=bot&permissions=0"
          , cache.user.id))
        .colour(Colour::fabled_pink())
        .description(&format!("I'm currently running {} - {}", &::BOT_NAME, &::VERSION))
        .title("Invite me to your server!")
        .author(|mut a| {
          a = a.name(&cache.user.name);
          if let Some(avatar) = cache.user.avatar_url() {
            a = a.icon_url(&avatar);
          }
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
    let _ = context.say("Unable to get startup time");
  }

});
