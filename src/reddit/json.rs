pub mod subreddit {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub data: Data,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub children: Vec<Child>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Child {
        pub data: ChildData,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChildData {
        pub title: String,
        pub selftext: String,
        pub permalink: String,
    }
}

pub mod post {
    use super::subreddit;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Response(pub subreddit::Response, pub Comments);

    #[derive(Debug, Deserialize)]
    pub struct Comments {
        pub data: Data,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub children: Vec<Child>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Child {
        pub data: ChildData,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChildData {
        pub body: Option<String>,
    }
}
