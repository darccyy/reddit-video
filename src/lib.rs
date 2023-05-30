/// Config, parsed from toml file
pub mod config;
/// Reddit API fetching
pub mod reddit;
/// Render video with ffmpeg
pub mod video;
/// Voice (TTS) API fetching
pub mod voice;

use std::{env, fs, path::Path};

use self::voice::Voice;

/// Text frames to render
type Text = String;

/// Convert a struct into a vector of text frames
trait ToTexts {
    /// Convert to vector of text frames
    fn to_texts(self) -> Vec<Text>;
}

impl<T: ToTexts> ToTexts for Vec<T> {
    fn to_texts(self) -> Vec<String> {
        self.into_iter().map(ToTexts::to_texts).flatten().collect()
    }
}

/// Fetch posts and comments, as texts
pub fn fetch_posts_or_comments(config: &config::Reddit) -> Vec<Text> {
    println!(
        "Fetching {} posts of r/{}...",
        reddit::sort_and_time(&config),
        config.subreddit
    );

    // Get posts
    let posts = reddit::fetch_posts(&config).expect("Failed to fetch posts");

    // Choose posts or comments
    let texts = if !config.comments {
        posts.to_texts()
    } else {
        // Select post to get comments of, with user input
        let parent_post = choose_parent_post(posts);

        // Get comments of post
        println!("Fetching top comments from chosen post...");
        let comments =
            reddit::fetch_comments(&config, &parent_post.link).expect("Failed to fetch comments");

        // Get texts, including parent post texts
        let mut texts = vec![parent_post.title.clone()];
        texts.append(&mut comments.to_texts());
        texts
    };

    // Limit amount of text frames
    texts
        .into_iter()
        .filter(|text| !text.is_empty())
        .take(config.limit as usize)
        .collect()
}

/// User select post to get comments of
fn choose_parent_post(posts: Vec<reddit::Post>) -> reddit::Post {
    inquire::Select::new("Which post to take comments from? (scroll for more)", posts)
        .with_page_size(12)
        .prompt()
        .expect("Error reading input")
}

/// Create temp directory, empty contents, and return path
pub fn get_empty_temp_dir() -> String {
    // Path to directory
    let temp = env::temp_dir().to_string_lossy().to_string();
    let name = env!("CARGO_PKG_NAME");
    let dir = format!("{temp}/{name}");

    // Remove and re-create
    if Path::new(&dir).exists() {
        fs::remove_dir_all(&dir).expect("Failed to remove temp dir");
    }
    fs::create_dir(&dir).expect("Failed to create temp dir");

    // Create subfolders
    let folders = &["audio"];
    for folder in folders {
        fs::create_dir(format!("{dir}/{folder}")).expect("Failed to create folder in temp dir");
    }

    dir
}

/// Save voices to temp directory, create list file for ffmpeg
pub fn save_voices(voices: &[Voice], dir: &str) {
    let mut inputs_file = Vec::new();
    for (i, voice) in voices.iter().enumerate() {
        // Save audio to file
        fs::write(format!("{dir}/audio/{i}.mp3"), &voice.bytes).expect("Failed to save voice file");
        // Add audio to list
        inputs_file.push(format!("file 'audio/{}.mp3'", i));
    }
    // Save list file
    fs::write(format!("{dir}/voices.txt"), inputs_file.join("\n"))
        .expect("Failed to save inputs file");
}
