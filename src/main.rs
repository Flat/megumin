#[macro_use]
extern crate serenity;
extern crate app_dirs;
extern crate chrono;
extern crate typemap;
extern crate kitsu_io;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serenity::Client;
use serenity::ext::framework::help_commands;
use chrono::*;
use app_dirs::*;
use std::env;
use std::collections::HashMap;
use typemap::Key;

mod commands;
mod util;

// Create a struct for keeping track of when we started
pub struct Uptime;
impl Key for Uptime {
    type Value = HashMap<String, DateTime<UTC>>;
}

// Get variables from the cargo build system
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BOT_NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

// Setup our app information for storing preferences
const APP_INFO: AppInfo = AppInfo {
    name: BOT_NAME,
    author: AUTHORS,
};

fn main() {
    // Get our bot token from the environment expected as DISCORD_TOKEN
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token);

    // Initiate our struct into the context.data
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<Uptime>(HashMap::default());
    }

    client.on_ready(|context, ready| {
        println!("{} is here!", ready.user.name);

        // Add the uptime as the time we connected succesfully
        let mut data = context.data.lock().unwrap();
        let uptime = data.get_mut::<Uptime>().unwrap();
        uptime.entry(String::from("boot")).or_insert_with(|| UTC::now());
    });

    // Setup bot framework with supported commands
    client.with_framework(|f| {
        f.configure(|c| {
                c.allow_whitespace(true)
                    .on_mention(true)
                    .prefix("%")
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => (),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            .group("Weeb", |g| {
                g.command("anime", |c| {
                        c.exec(commands::weeb::anime)
                            .min_args(1)
                            .desc("Searches kitsu.io for the specified anime")
                    })
                    .command("manga", |c| {
                        c.exec(commands::weeb::manga)
                            .min_args(1)
                            .desc("Searches kitsu.io for the specified manga")
                    })
                    .command("kitsuprofile", |c| {
                        c.exec(commands::weeb::kitsu_user)
                            .min_args(1)
                            .desc("Seaches kitsu.io for the specified user")
                    })
            })
            .group("General", |g| {
                g.command("about", |c| {
                        c.exec(commands::general::about)
                            .desc("Information about this software")
                    })
                    .command("help", |c| c.exec_help(help_commands::with_embeds))
                    .command("info", |c| {
                        c.exec(commands::general::info)
                            .desc(&format!("Replies with information about {}", BOT_NAME))
                    })
            })
            .group("Administration", |g| {
                g.command("setowner", |c| {
                        c.exec(commands::admin::set_owner)
                            .desc("Sets the owner of this bot to the user that calls this command")
                    })
                    .command("owner", |c| {
                        c.exec(commands::admin::view_owner)
                            .desc("Returns the user id of the owner of this bot")
                    })
                    .command("playing", |c| {
                        c.exec(commands::admin::set_game)
                            .check(util::owner_check)
                            .min_args(1)
                    })
                    .command("clearplaying", |c| {
                        c.exec(commands::admin::no_game)
                            .check(util::owner_check)
                    })
            })
            // .group("Audio", |g| {
            //     g.command("play", |c| {
            //             c.exec(commands::audio::play)
            //                 .desc("Plays the given youtube or soundcloud url")
            //         })
            //         .command("stop", |c| {
            //             c.exec(commands::audio::stop)
            //                 .desc("stops the currently playing music")
            //         })
            // })
    });

    if let Err(why) = client.start() {
        // rip bot
        println!("Client error: {:?}", why);
    }
}
