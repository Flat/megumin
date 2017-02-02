# Megumin

A general purpose discord bot built with the serenity library. Programmed in Rust and made with love.

## Commands

| Weeb | Administration | General |
| :---: | :---: | :---: |
| anime | nogame | about |
| | setowner | help |
| | owner | | info|
| | disown | | |
| | setgame | | | 

## Building

### Linux

#### Debug
* `git clone https://github.com/flat/megumin`
* `cargo build` or `cargo run`

#### Release
* `git clone https://github.com/flat/megumin`
* `cargo build --release` or `cargo run --release`

## Using

### Bot Token
Megumin only works as a bot user. To run an environment variable with the name `DISCORD_TOKEN` and the value of a bot token issued by the discord api is needed.

### Settings
Currently the only thing stored in the settings file is the bot owner's userid.

### Linux
`~/.config/megumin/bot-settings/owner.prefs.json`