// ref: https://github.com/benwis/bisky/blob/main/examples/honk_bot/src/main.rs

use bisky::atproto::{Client, Session};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
use bisky::storage::{File, Storage as _};
use chrono::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// A file to store JSON Web Tokens in
    #[clap(index = 1)]
    storage: PathBuf,
    /// Which atproto service to connect to
    #[clap(index = 2)]
    service: Url,
    /// Username to log in with
    #[clap(index = 3)]
    username: String,
    /// Password to log in with
    #[clap(index = 4)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let mut storage = File::<Session>::new(args.storage);
    if storage.get().await.is_err() {
        Client::login(&args.service, &args.username, &args.password, &mut storage)
            .await
            .unwrap();
    }
    let post = Post {
        text: "Test post from Rust!".to_string(),
        created_at: Utc::now(),
    };

    let mut client = Bluesky::new(Client::new(args.service, storage).await.unwrap());

    match client.me().post(post).await {
        Ok(success) => {
            println!("Successfully posted: {} - {}", success.cid, success.uri)
        }
        Err(e) => {
            println!("I failed somehow: {:?}", e)
        }
    }
}
