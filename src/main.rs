use keynergy::layout::Layout;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;
use std::fs;

struct Bot {
    layouts: HashMap<String, Layout>,
}

#[async_trait]
impl EventHandler for Bot {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!layout") {
            let split: Vec<&str> = msg.content.split_whitespace().collect();
            if split.len() == 1 {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "You need to specify a layout.")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                    return;
                }
            }
            let result = self.layouts.get(&split[1..].join(" ").to_ascii_lowercase());
            match result {
                None => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Layout not in DB.").await {
                        println!("Error sending message: {:?}", why);
                        return;
                    }
                }
                Some(l) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, print_layout(l)).await {
                        println!("Error sending message: {:?}", why);
                        return;
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn display_matrix(m: &Vec<Vec<char>>) -> String {
    let mut s = String::new();
    for r in m.iter() {
        for c in r.iter() {
            s.push(*c);
            s.push(' ');
        }
        s.push('\n');
    }
    s
}

fn print_layout(l: &Layout) -> String {
    std::format!(
        "**{}**\nCreated by {}\n```\n{}\n```",
        l.name,
        l.author,
        display_matrix(&l.formats.standard.as_ref().unwrap().matrix)
    )
}

#[tokio::main]
async fn main() {
    let mut bot = Bot {
        layouts: HashMap::new(),
    };
    let dir = fs::read_dir("./layouts").unwrap();
    print!("Reading layouts...");
    for f in dir.into_iter() {
        // this is horrible but I'm lazy
        // TODO fix
        let mut l = Layout::load(f.unwrap().path().to_str().unwrap().to_string()).unwrap();
        if l.link == Some("".to_string()) {
            l.link = None;
        }
        bot.layouts.insert(l.name.to_ascii_lowercase(), l);
    }
    println!("done");
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
