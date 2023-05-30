/// Create `Deserialize`-able structs for JSON responses
macro_rules! json {
    ( $(
       $struct:ident $tt:tt
    )* ) => { $(
        json!(@single $struct $tt);
    )* };

    (@single $struct:ident { $( $key:ident : $type:ty),* $(,)? } ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        pub struct $struct {
            $( pub $key: $type, )*
        }
    };

    (@single $struct:ident ( $( $type:ty ),* $(,)? ) ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        pub struct $struct (
            $( pub $type, )*
        );
    };
}

/// Response of posts of subreddit
pub mod subreddit {
    json! {
        Response {
            data: Data,
        }
        Data {
            children: Vec<Child>,
        }
        Child {
            data: ChildData,
        }
        ChildData {
            title: String,
            selftext: String,
            permalink: String,
            score: i32,
            num_comments: u32,
        }
    }
}

/// Response of comments of post
pub mod post {
    json! {
        Response (
            super::subreddit::Response,
            Comments,
        )
        Comments {
            data: Data,
        }
        Data {
            children: Vec<Child>,
        }
        Child {
            data: ChildData,
        }
        ChildData {
            body: Option<String>,
        }
    }
}
