#[macro_use]
mod macros;

destructs! {
    #[derive(Debug)]
    pub struct Config: Default {
        assets: Assets = Default,
        out: Out = Default,
        reddit: Reddit = Default,
        voice: Voice = Default,
    }

    #[derive(Debug)]
    pub struct Assets: Default {
        background: String = "background.mp4",
        watermark: Option<String> = None,
    }

    #[derive(Debug)]
    pub struct Out: Default {
        name: String = "video.mp4",
    }

    #[derive(Debug)]
    pub struct Reddit: Default {
        subreddit: String = "askreddit",
        sort: String = "top",
        time: String = "month",
        comments: bool = true,
        limit: usize = 500usize,
    }

    #[derive(Debug)]
    pub struct Voice: Default {
        language: String = "en-GB",
        gender: String = "male",
        pitch: f32 = 0.5,
        rate: f32 = 0.5,
    }
}

impl std::str::FromStr for Config {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
