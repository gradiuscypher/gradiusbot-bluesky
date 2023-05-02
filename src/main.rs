// ref: https://github.com/benwis/bisky/blob/main/examples/honk_bot/src/main.rs
use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    storage: PathBuf,
    #[clap(index = 2)]
    service: Url,
    #[clap(index = 3)]
    username: String,
    #[clap(index = 4)]
    password: String,
    #[clap(index = 5)]
    post_text: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let storage = Arc::new(File::<UserSession>::new(args.storage));

    let mut client = ClientBuilder::default()
        .session(None)
        .storage(storage)
        .build()
        .unwrap();
    client
        .login(&args.service, &args.username, &args.password)
        .await
        .unwrap();
    let mut bsky = Bluesky::new(client);

    bsky.me()
        .unwrap()
        .post(Post {
            text: args.post_text,
            created_at: chrono::Utc::now(),
            embed: None,
            reply: None,
            rust_type: None,
        })
        .await
        .unwrap();
}
