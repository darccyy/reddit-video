use std::fs;

use reddit_video::{
    config::Config, fetch_posts_or_comments, get_empty_temp_dir, save_voices, video, voice,
};
use stilo::println_styles;

macro_rules! info {
    ( $($tt:tt)* ) => {
        println_styles!( $($tt)*: Cyan );
    };
}

fn main() {
    println_styles!(" === Reddit Video === ": Cyan + bold);

    const CONFIG_FILENAME: &str = "./config.toml";

    let config: Config = fs::read_to_string(CONFIG_FILENAME)
        .unwrap_or(String::new())
        .parse()
        .expect("Failed to parse config file");

    println!("{:#?}", config);

    let temp_dir = get_empty_temp_dir();

    info!("Fetching content...");
    let texts = fetch_posts_or_comments(&config.reddit);
    // let texts = vec![
    //     "this is some text".to_string(),
    //     // "some more text\nactually".to_string(),
    //     // "blah blah".to_string(),
    // ];

    println!("{:#?}", texts);

    info!("Creating voices...");
    let voices = voice::create_voices(&config.voice, texts).expect("Failed to fetch voices");
    info!("Saving voices...");
    save_voices(&voices, &temp_dir);

    info!("Concatenating audio...");
    video::concat_voices(&config, &temp_dir);

    info!("Adding audio to video...");
    video::apply_video_audio(&config, &temp_dir);

    info!("Rendering video with text...");
    video::render_video(&config, &temp_dir, &voices);

    println_styles!("Completed successfully!": Green + bold);
}
