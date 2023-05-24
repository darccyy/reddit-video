use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "Default::default")]
    pub assets: Assets,
    #[serde(default = "Default::default")]
    pub out: Out,
    #[serde(default = "Default::default")]
    pub content: Content,
    #[serde(default = "Default::default")]
    pub voice: Voice,
}

impl FromStr for Config {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    #[serde(default = "background")]
    pub background: String,
    #[serde(default = "watermark")]
    pub watermark: Option<String>,
}

fn background() -> String {
    String::from("background.mp4")
}
fn watermark() -> Option<String> {
    None
}
impl Default for Assets {
    fn default() -> Self {
        Self {
            background: background(),
            watermark: watermark(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Out {
    #[serde(default = "name")]
    pub name: String,
}

fn name() -> String {
    String::from("video.mp4")
}
impl Default for Out {
    fn default() -> Self {
        Self { name: name() }
    }
}

#[derive(Debug, Deserialize)]
pub struct Content {
    #[serde(default = "subreddit")]
    pub subreddit: String,
    #[serde(default = "sort")]
    pub sort: String,
    #[serde(default = "time")]
    pub time: String,
    #[serde(default = "comments")]
    pub comments: bool,
    #[serde(default = "limit")]
    pub limit: u32,
}

fn subreddit() -> String {
    String::from("askreddit")
}
fn sort() -> String {
    String::from("top")
}
fn time() -> String {
    String::from("month")
}
fn comments() -> bool {
    true
}
fn limit() -> u32 {
    5
}
impl Default for Content {
    fn default() -> Self {
        Self {
            subreddit: subreddit(),
            sort: sort(),
            time: time(),
            comments: comments(),
            limit: limit(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    #[serde(default = "language")]
    pub language: String,
    #[serde(default = "gender")]
    pub gender: String,
    #[serde(default = "pitch")]
    pub pitch: f32,
    #[serde(default = "rate")]
    pub rate: f32,
}

fn language() -> String {
    String::from("en-GB")
}
fn gender() -> String {
    String::from("male")
}
fn pitch() -> f32 {
    0.5
}
fn rate() -> f32 {
    0.5
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            language: language(),
            gender: gender(),
            pitch: pitch(),
            rate: rate(),
        }
    }
}
