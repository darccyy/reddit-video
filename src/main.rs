use std::fs;

use reddit_video::{config::Config, fetch_posts_or_comments};

fn main() {
    const CONFIG_FILENAME: &str = "./config.toml";

    let config: Config = fs::read_to_string(CONFIG_FILENAME)
        .unwrap_or(String::new())
        .parse()
        .expect("Failed to parse config file");

    println!("{:#?}", config);

    println!(" === Reddit Video === ");

    let texts = fetch_posts_or_comments(&config.reddit);

    println!("{:#?}", texts);
}
