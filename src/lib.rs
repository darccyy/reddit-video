pub mod config;
pub mod reddit;
pub mod voice;

mod number;

use std::{env, fs, path::Path};

use number::format_number;
use voice::Voice;

trait ToTextFrames {
    fn to_text_frames(self) -> Vec<String>;
}

impl<T: ToTextFrames> ToTextFrames for Vec<T> {
    fn to_text_frames(self) -> Vec<String> {
        self.into_iter()
            .map(ToTextFrames::to_text_frames)
            .flatten()
            .collect()
    }
}

pub fn fetch_posts_or_comments(config: &config::Reddit) -> Vec<String> {
    println!(
        "Fetching {} posts of r/{}...",
        reddit::sort_and_time(&config),
        config.subreddit
    );

    let posts = reddit::fetch_posts(&config).expect("Failed to fetch posts");

    let texts = if !config.comments {
        posts.to_text_frames()
    } else {
        let parent_post = choose_parent_post(posts);

        println!("Fetching top comments from chosen post...");
        let comments =
            reddit::fetch_comments(&config, &parent_post.link).expect("Failed to fetch comments");

        let mut texts = vec![parent_post.title.clone()];
        texts.append(&mut comments.to_text_frames());
        texts
    };

    texts
        .into_iter()
        .filter(|text| !text.is_empty())
        .take(config.limit as usize)
        .collect()
}

fn choose_parent_post(posts: Vec<reddit::Post>) -> reddit::Post {
    inquire::Select::new("Which post to take comments from? (scroll for more)", posts)
        .with_page_size(12)
        .prompt()
        .expect("Error reading input")
}

pub fn get_empty_temp_dir() -> String {
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

pub fn save_voices(voices: &[Voice], dir: &str) {
    let mut inputs_file = Vec::new();

    for (i, voice) in voices.iter().enumerate() {
        fs::write(format!("{dir}/audio/{i}.mp3"), &voice.bytes).expect("Failed to save voice file");

        inputs_file.push(format!("file 'audio/{}.mp3'", i));
    }

    fs::write(format!("{dir}/voices.txt"), inputs_file.join("\n"))
        .expect("Failed to save inputs file");
}
