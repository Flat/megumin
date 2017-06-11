use regex::Regex;
use serenity::ext::voice;

command!(play(context, message, args){
  let to_play = args.join(" ");
  let youtube = Regex::new(r"https?://(www\.youtube\.com|youtu\.be)(/watch\?v=|/).{11}").unwrap();
  let soundcloud = Regex::new(r"https?://soundcloud\.com/.*/.*").unwrap();
  let search = !(youtube.is_match(&to_play) || soundcloud.is_match(&to_play));

  let voice_info = match ::util::get_user_audio_channel(message) {
    Some((x, y)) => ((x, y)),
    None => return Ok(()),
  };

  let guild_id = voice_info.0;
  let channel_id = match voice_info.1 {
    Some(x) => x,
    None => return Ok(()),
  };

  let mut shard = context.shard.lock().unwrap();
  shard.manager.join(guild_id, channel_id);



  if search {

  } else {
    if let Some(handler) = shard.manager.get(guild_id) {
      let source = match voice::ytdl(&to_play) {
        Ok(source) => source,
        Err(why) => {
          println!("Err starting source: {:?}", why);
          return Ok(());
        },
      };
    handler.play(source);
  }
}

});

command!(stop(context, message, _args){
  let voice_info = match ::util::get_user_audio_channel(message) {
    Some((x, y)) => ((x, y)),
    None => return Ok(()),
  };

  let guild_id = voice_info.0;

  let mut shard = context.shard.lock().unwrap();
    let has_handler = shard.manager.get(guild_id).is_some();

    if has_handler {
        shard.manager.remove(guild_id);
    }

});
