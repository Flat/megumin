use kitsu_io;
use serenity::utils::Colour;

// Searches kitsu.io for a passed string
command!(anime(_context, message, args) {
  // Accept all arguments as one search string
  let search = args.join(" ");
  // Let the user know we're working on it.
  let mut msg = match message.channel_id.say(&format!("Searching kitsu.io for {}", &search)){
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
        // Round rating to hundredths
        Some(x) => (((x * 100_f64).round())/100_f64).to_string(),
        None => "N/A".to_owned(),
      };
      let anime_type = match anime.attributes.kind.name(){
        Ok(x) => x,
        Err(_) => "N/A".to_owned(),
      };
      let anime_airing_status = anime.attributes.airing_status();
      let anime_airing_status_name = anime_airing_status.name();
      let anime_episode_count = match anime.attributes.episode_count {
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      };
      let anime_start_date = &anime.attributes.start_date;
      let anime_end_date = match anime.attributes.end_date {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let anime_cover_image = match anime.attributes.cover_image {
        Some(ref x) => 
          match x.largest(){
            Some(y) => y,
            None => "N/A",
          },
        None => "N/A",
      };

      let anime_poster_image = match anime.attributes.poster_image.largest(){
        Some(x) => x,
        None => "N/A",
      };

      // Update the message with our new found knowledge
      let _ = match msg.edit(|mut m| { m = m.content(""); m = m.embed(|mut e| { e = e
        .author(|mut a| {
          a = a.name("Kitsu.io");
          // Use kitsu's android favicon as an avatar (This might break in the future)
          a = a.icon_url("https://kitsu.io/android-chrome-192x192.png");
          a
        })
        .url(&anime.url())
        .colour(Colour::from_rgb(51,37,50))
        .description(&anime_synopsis)
        .title(&anime_title)
        .thumbnail(anime_poster_image)
        .field(|f| f
          .inline(true)
          .name("Average Rating")
          .value(&anime_average_rating)
          )
        .field(|f| f
          .inline(true)
          .name("Type")
          .value(&anime_type)
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
          .value(anime_start_date)
          )
        .field(|f| f
          .inline(true)
          .name("End Date")
          .value(&anime_end_date)
          )
        .field(|f| f
          .inline(true)
          .name("Status")
          .value(anime_airing_status_name)
          );
        if anime_cover_image != "N/A" {
          e = e.image(&anime_cover_image);
        }
        if anime_poster_image != "N/A" {
          e = e.thumbnail(anime_poster_image);
        }
        e});
        m
        }){
        Ok(msg) => msg,
        Err(why) => {
          // Something went wrong creating the embed
          println!("{:?}", why);
          let _ = msg.edit(|m| m.content("Failed to submit embedded message."));
          return Ok(());
        },
      };

    } else {
      // Something went wrong getting the first result
      let _ = msg.edit(|m| m.content("Failed to retrieve information."));
    }
}
  else {
    // Something went wrong with the request to the api
    let _ = msg.edit(|m| m.content("Failed to retrieve information."));
  }
});


// Search kitsu.io for a manga
command!(manga(_context, message, args){
   // Accept all arguments as one search string
  let search = args.join(" ");
  // Let the user know we're working on it.
  let mut msg = match message.channel_id.say(&format!("Searching kitsu.io for {}", &search)){
    Ok(msg) => msg,
    Err(_) => return Ok(()),
  };
  // Make the request to the api and make sure it's Ok
  if let Ok(result) = kitsu_io::search_manga(|f| f.filter("text", &search)){
    // Grab the first result. 
    if let Some(manga) = result.data.get(0) {
      // Parse all the attributes to their own variables (A large amount of the API returns are optional)
      let manga_title = &manga.attributes.canonical_title;
      let mut manga_synopsis = manga.attributes.synopsis.to_owned();
      if &manga_synopsis == "" {
        manga_synopsis = "N/A".to_owned();
      }
      let manga_type = &manga.attributes.kind;
      let manga_average_rating = match manga.attributes.average_rating {
        // Round rating to hundredths
        Some(x) => (((x * 100_f64).round())/100_f64).to_string(),
        None => "N/A".to_owned(),
      };
      let manga_serialization = match manga.attributes.serialization {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let manga_volume_count = match manga.attributes.volume_count {
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      };
      let manga_chapter_count = match manga.attributes.chapter_count{
        Some(x) => x.to_string(),
        None => "N/A".to_owned(),
      };
      let manga_start_date = match manga.attributes.start_date {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let manga_end_date = match manga.attributes.end_date {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let manga_cover_image = match manga.attributes.cover_image {
        Some(ref x) => 
          match x.largest() {
            Some(y) => y,
            None => "N/A",
          },
        None => "N/A",
      };

      let manga_poster_image = match manga.attributes.poster_image.largest(){
        Some(x) => x,
        None => "N/A",
      };

      // Update the message with our new found knowledge
      let _ = match msg.edit(|mut m| { m = m.content(""); m = m.embed(|mut e| { e = e
        .author(|mut a| {
          a = a.name("Kitsu.io");
          // Use kitsu's android favicon as an avatar (This might break in the future)
          a = a.icon_url("https://kitsu.io/android-chrome-512x512-5b1696ec45da4d52b7dcdedb969ffa4e.png");
          a
        })
        .url(&manga.url())
        .colour(Colour::from_rgb(51,37,50))
        .description(&manga_synopsis)
        .title(&manga_title)
        .field(|f| f
          .inline(true)
          .name("Average Rating")
          .value(&manga_average_rating)
          )
        .field(|f| f
          .inline(true)
          .name("Type")
          .value(&format!("{:?}", manga_type))
          )
        .field(|f| f
          .inline(true)
          .name("Serialization")
          .value(&manga_serialization)
          )
        .field(|f| f
          .inline(true)
          .name("Volumes")
          .value(&manga_volume_count)
          )
        .field(|f| f
          .inline(true)
          .name("Chapters")
          .value(&manga_chapter_count)
          )
        .field(|f| f
          .inline(true)
          .name("Start Date")
          .value(&manga_start_date)
          )
        .field(|f| f
          .inline(true)
          .name("End Date")
          .value(&manga_end_date)
          );
        if manga_cover_image != "N/A" {
          e = e.image(&manga_cover_image);
        }
        if manga_poster_image != "N/A" {
          e = e.thumbnail(manga_poster_image);
        }
        e});
        m
        }) {
        Ok(msg) => msg,
        Err(why) => {
          // Something went wrong creating the embed
          println!("{:?}", why);
          let _ = msg.edit(|m| m.content("Failed to submit embedded message."));
          return Ok(());
        },
      };

    } else {
      // Something went wrong getting the first result
      let _ = msg.edit(|m| m.content("Failed to retrieve information."));
    }
}
  else {
    // Something went wrong with the request to the api
    let _ = msg.edit(|m| m.content("Failed to retrieve information."));
  }
});

// Search kitsu.io for a user
command!(kitsu_user(_context, message, args){
   // Accept all arguments as one search string
  let search = args.join(" ");
  // Let the user know we're working on it.
  let mut msg = match message.channel_id.say(&format!("Searching kitsu.io for {}", &search)){
    Ok(msg) => msg,
    Err(_) => return Ok(()),
  };
  // Make the request to the api and make sure it's Ok
  if let Ok(result) = kitsu_io::search_users(|f| f.filter("query", &search)){
    // Grab the first result. 
    if let Some(user) = result.data.get(0) {
      // Parse all the attributes to their own variables (A large amount of the API returns are optional)
      let user_name = &user.attributes.name;
      let mut user_bio = user.attributes.bio.to_owned();
      if &user_bio == "" {
        user_bio = "N/A".to_owned();
      }
      let mut user_about = user.attributes.about.to_owned();
      if &user_about == "" {
        user_about = "N/A".to_owned();
      }
      let user_birthday = match user.attributes.birthday {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };
      let user_cover_image = match user.attributes.cover_image {
        Some(ref x) => 
          match x.largest() {
            Some(y) => y,
            None => "N/A",
          },
        None => "N/A",
      };
      let user_avatar = match user.attributes.avatar {
        Some(ref x) => 
          match x.largest() {
            Some(y) => y,
            None => "N/A",
          },
        None => "N/A",
      };
      let user_followers = user.attributes.followers_count.to_string();
      let user_following = user.attributes.following_count.to_string();
      let user_gender = match user.attributes.gender {
        Some(ref x) => x.to_owned(),
        None => "N/A".to_owned(),
      };


      // Update the message with our new found knowledge
      let _ = match msg.edit(|mut m| { m = m.content(""); m = m.embed(|mut e| { e = e
        .author(|mut a| {
          a = a.name("Kitsu.io");
          // Use kitsu's android favicon as an avatar (This might break in the future)
          a = a.icon_url("https://kitsu.io/android-chrome-192x192.png");
          a
        })
        .url(&user.url())
        .colour(Colour::from_rgb(51,37,50))
        .description(&user_about)
        .title(&user_name)
        .field(|f| f
          .inline(true)
          .name("Gender")
          .value(&user_gender)
          )
        .field(|f| f
          .inline(true)
          .name("Birthday")
          .value(&user_birthday)
          )
        .field(|f| f
          .inline(true)
          .name("Followers")
          .value(&user_followers)
          )
        .field(|f| f
          .inline(true)
          .name("Following")
          .value(&user_following)
          )
        .field(|f| f
          .inline(true)
          .name("Bio")
          .value(&user_bio)
          );
        if user_cover_image != "N/A" {
          e = e.image(user_cover_image);
        }
        if user_avatar != "N/A" {
          e = e.thumbnail(user_avatar)
        }
        e});
        m
        }){
        Ok(msg) => msg,
        Err(why) => {
          // Something went wrong creating the embed
          println!("{:?}", why);
          let _ = msg.edit(|m| m.content("Failed to submit embedded message."));
          return Ok(());
        },
      };

    } else {
      // Something went wrong getting the first result
      let _ = msg.edit(|m| m.content("Failed to retrieve information."));
    }
}
  else {
    // Something went wrong with the request to the api
    let _ = msg.edit(|m| m.content("Failed to retrieve information."));
  }
});
