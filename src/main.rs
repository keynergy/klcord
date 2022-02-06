mod bot;
mod utility;

use bot::*;
use serenity::Client;
use std::fs;

#[tokio::main]
async fn main() {
    let bot = Bot::with_layouts_in_dir("layouts");

    let mut token = fs::read_to_string("token").expect("Error reading token file");
    token = token.trim().to_string();

    println!("Creating client...");
    let mut client = Client::builder(&token)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    println!("Starting client...");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
