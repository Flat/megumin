use regex::Regex;

command!(play(_context, message, args){
  let to_play = args.join(" ");
  let youtube = Regex::new(r"https?:\/\/www.youtube.com\/watch\?v=.{11}");
  let soundcloud = Regex::new(r"https?:\/\/soundcloud.com\/.*\/.*");
  let search = if(youtube.is_match(to_play) || soundcloud.is_match(to_play));

  

});