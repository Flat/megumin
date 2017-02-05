use kitsu_io;
use serenity::utils::Colour;

// Searches kitsu.io for a passed string
command!(anime(context, _message, args) {
  // Accept all arguments as one search string
  let search = args.join(" ");
  // Let the user know we're working on it.
  let mut msg = match context.say(&format!("Searching kitsu.io for {}", &search)){
    Ok(msg) => msg,
    Err(_) => return Ok(()),
  };
  // Make the request to the api and make sure it's Ok
  if let Ok(result) = kitsu_io::search_anime(|f| f.filter("text", &search)){
    // Grab the first result. 
    // TODO: Get the most relevent result instead of the first. 
    // Kitsu isn't great at not giving you obscure OVA's
    if let Some(anime) = result.data.get(0) {
      // Parse all the attributes to their own variables (A large amount of the API returns are optional)
      let anime_title = &anime.attributes.canonical_title;
      let anime_synopsis = &anime.attributes.synopsis;
      let anime_age_rating = match anime.attributes.age_rating {
        Some(ref x) => format!("{:?}", x),
        None => "N/A".to_owned(),
      };
      let anime_average_rating = match anime.attributes.average_rating {
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      };
      let anime_type = anime.attributes.kind;
      let anime_episode_count = match anime.attributes.episode_count {
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      };
      let anime_start_date = anime.attributes.start_date.to_owned();
      let anime_end_date = match anime.attributes.end_date {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let anime_cover_image = match anime.attributes.cover_image {
        Some(ref x) => x.original.to_owned().unwrap_or("N/A".to_owned()),
        None => "N/A".to_owned(),
      };

      let anime_poster_image = match anime.attributes.poster_image.largest(){
        Some(x) => x,
        None => "N/A",
      };

      // Update the message with our new found knowledge
      let _ = match msg.edit("", |mut e| { e = e
        .author(|mut a| {
          a = a.name("Kitsu.io");
          // Use kitsu's android favicon as an avatar (This might break in the future)
          a = a.icon_url("https://kitsu.io/android-chrome-192x192.png");
          a
        })
        .url(&anime.url())
        .colour(Colour::from_rgb(51,37,50))
        .description(&anime_synopsis)
        .title(&format!("{}", &anime_title))
        .thumbnail(anime_poster_image)
        .field(|f| f
          .inline(true)
          .name("Average Rating")
          .value(&anime_average_rating)
          )
        .field(|f| f
          .inline(true)
          .name("Type")
          .value(&format!("{:?}", anime_type))
          )
        .field(|f| f
          .inline(true)
          .name("Age Rating")
          .value(&anime_age_rating)
          )
        .field(|f| f
          .inline(true)
          .name("Episodes")
          .value(&anime_episode_count)
          )
        .field(|f| f
          .inline(true)
          .name("Start Date")
          .value(&anime_start_date)
          )
        .field(|f| f
          .inline(true)
          .name("End Date")
          .value(&anime_end_date)
          );
        if &anime_cover_image != "N/A" {
          e = e.image(&anime_cover_image);
        }
        e}){
        Ok(msg) => msg,
        Err(why) => {
          // Something went wrong creating the embed
          println!("{:?}", why);
          let _ = msg.edit("Failed to submit embedded message.", |e| e);
          return Ok(());
        },
      };

    } else {
      // Something went wrong getting the first result
      let _ = msg.edit("Failed to retrieve information.", |e| e);
    }
}
  else {
    // Something went wrong with the request to the api
    let _ = msg.edit("Failed to retrieve information.", |e| e);
  }
});
