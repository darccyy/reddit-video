use std::fs;

use reddit_video::{config::Config, get_empty_temp_dir, save_voices, video, voice};

fn main() {
    const CONFIG_FILENAME: &str = "./config.toml";

    let config: Config = fs::read_to_string(CONFIG_FILENAME)
        .unwrap_or(String::new())
        .parse()
        .expect("Failed to parse config file");

    println!("{:#?}", config);

    println!(" === Reddit Video === ");

    let temp_dir = get_empty_temp_dir();

    // let texts = fetch_posts_or_comments(&config.reddit);
    let texts = vec![
        "this is some text".to_string(),
        // "some more text\nactually".to_string(),
        // "blah blah".to_string(),
    ];

    println!("{:#?}", texts);

    println!("Creating voices...");
    let voices = voice::create_voices(&config.voice, texts).expect("Failed to fetch voices");
    save_voices(&voices, &temp_dir);

    println!("Concatenating audio...");
    video::concat_voices(&config, &temp_dir);
    println!("Adding audio to video...");
    video::apply_video_audio(&config, &temp_dir);
    println!("Rendering video with text...");
    video::render_video(&config, &temp_dir, &voices);

    println!("done");
}
