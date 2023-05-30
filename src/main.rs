use std::{env, fs, path::Path};

use reddit_video::{config::Config, fetch_posts_or_comments, voice::create_voices};

fn main() {
    const CONFIG_FILENAME: &str = "./config.toml";

    let config: Config = fs::read_to_string(CONFIG_FILENAME)
        .unwrap_or(String::new())
        .parse()
        .expect("Failed to parse config file");

    println!("{:#?}", config);

    println!(" === Reddit Video === ");

    let temp_dir = get_empty_temp_dir();

    let texts = fetch_posts_or_comments(&config.reddit);

    println!("{:#?}", texts);

    println!("Creating voices...");
    let voices = create_voices(&config.voice, &texts).expect("Failed to fetch voices");

    let mut inputs_file = Vec::new();
    for (i, voice) in voices.iter().enumerate() {
        fs::write(format!("{}/audio/{}.mp3", temp_dir, i), &voice.bytes)
            .expect("Failed to save voice file");

        inputs_file.push(format!("file 'audio/{}.mp3'", i));
    }

    fs::write(temp_dir + "/voices.txt", inputs_file.join("\n"))
        .expect("Failed to save inputs file");

    println!("done");
}

fn get_empty_temp_dir() -> String {
    let temp = env::temp_dir().to_string_lossy().to_string();
    let name = env!("CARGO_PKG_NAME");

    let dir = format!("{temp}/{name}");

    if Path::new(&dir).exists() {
        fs::remove_dir_all(&dir).expect("Failed to remove temp dir");
    }
    fs::create_dir(&dir).expect("Failed to create temp dir");

    let folders = &["audio"];
    for folder in folders {
        fs::create_dir(format!("{dir}/{folder}")).expect("Failed to create folder in temp dir");
    }

    dir
}
