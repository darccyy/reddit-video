pub mod config;
pub mod reddit;
pub mod voice;

mod number;

use number::format_number;

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

    let texts = texts.into_iter().filter(|text| !text.is_empty());

    let texts = texts.take(config.limit as usize).collect();

    texts
}

fn choose_parent_post(posts: Vec<reddit::Post>) -> reddit::Post {
    inquire::Select::new("Which post to take comments from? (scroll for more)", posts)
        .with_page_size(12)
        .prompt()
        .expect("Error reading input")
}
