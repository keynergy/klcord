use keynergy::Layout;
use layoutexport::XkbLayout;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{collections::HashMap, process::Command};

use crate::utility::*;

pub struct Bot {
    layouts: HashMap<String, Layout>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            layouts: HashMap::new(),
        }
    }

    pub fn with_layouts_in_dir(dir: &str) -> Bot {
        let mut bot = Bot::new();
        print!("Reading layouts... ");
        bot.layouts = get_layouts_from_dir(dir);
        println!("Done!");
        bot
    }
}

#[async_trait]
impl EventHandler for Bot {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content.starts_with("!layouts") {
        	send_message(&ctx, &msg, format!(
        		"this list would be too chonky. Here, take this instead:\n
        		https://github.com/keynergy/klcord/tree/main/layouts"
       		)).await;
       	} else if msg.content.starts_with("!layout") {
            let split: Vec<&str> = msg.content.split_whitespace().collect();
            if split.len() == 1 {
                send_message(&ctx, &msg, "Usage: `!layout <layout name>`").await;
            } else {
                let mut name = split[1..].join(" ").to_ascii_lowercase();

                if name == *"mtgap" {
                    name = String::from("mtgap30");
                }

                match self.layouts.get(&name) {
                    None => match &name[..] {
                        "taipo" => {
                            let path = vec!["taipo.png"];
                            let result = msg
                                .channel_id
                                .send_files(&ctx, path, |m| m.content("Taipo | Created by whorf\nhttps://inkeys.wiki/en/keymaps/taipo"))
                                .await;
                            result.unwrap();
                        }
                        _ => {
                            send_message(
                                &ctx,
                                &msg,
                                format!(
                                    "This layout does not exist.\n\
				     Did you mean {}?",
                                    closest_match(
                                        name,
                                        &self
                                            .layouts
                                            .keys()
                                            .map(|x| x.as_str())
                                            .collect::<Vec<&str>>()[..]
                                    )
                                ),
                            )
				.await;
                        }
                    },

                    Some(l) => {
                        send_message(&ctx, &msg, print_layout(l)).await;
                    }
                }
            }
        } else if msg.content.starts_with("!translate") {
            let split: Vec<&str> = msg.content.split_whitespace().collect();
            if split.len() < 4 {
                send_message(
                    &ctx,
                    &msg,
                    "Usage: `!translate <from layout> <to layout> a sentence`",
                )
                    .await;
            } else {
                let f = &split[1].replace("_", " ").to_ascii_lowercase();
                let t = &split[2].replace("_", " ").to_ascii_lowercase();
                println!("{} {}", f, t);
                let from = match self.layouts.get(f) {
                    Some(x) => x.formats.standard.as_ref().unwrap(),
                    None => {
                        send_message(&ctx, &msg, format!("Layout {} does not exist.", f)).await;
                        return;
                    }
                };

                let to = match self.layouts.get(t) {
                    Some(x) => x.formats.standard.as_ref().unwrap(),
                    None => {
                        send_message(&ctx, &msg, format!("Layout {} does not exist.", t)).await;
                        return;
                    }
                };

                let text = split[3..].join(" ");
                let mut newtext = String::new();
                for c in text.chars() {
                    newtext.push(match from.map.get(&c.to_ascii_lowercase()) {
                        None => c,
                        Some(x) => *to.pos_key_unsafe(*x),
                    });
                }
                send_message(&ctx, &msg, newtext).await;
            }
        } else if msg.content.starts_with("!xkb") {
            let split: Vec<&str> = msg.content.split_whitespace().collect();
            if split.len() == 1 {
                send_message(&ctx, &msg, "Usage: `!xkb LAYOUT`").await;
            }
            let name = split[1..].join(" ").to_ascii_lowercase();
            match self.layouts.get(&name) {
                None => {
                    send_message(
                        &ctx,
                        &msg,
                        format!(
                            "This layout does not exist.\n\
			     Did you mean {}?",
                            closest_match(
                                name,
                                &self
                                    .layouts
                                    .keys()
                                    .map(|x| x.as_str())
                                    .collect::<Vec<&str>>()[..]
                            )
                        ),
                    )
                    .await;
                }
                Some(l) => {
                    let xkb = match XkbLayout::from(l) {
                        Ok(x) => x,
                        Err(_) => return,
                    };

                    send_message(&ctx, &msg, format!("```\n{}\n```", xkb.content)).await;
                }
            }; 
        } else if msg.content.starts_with("!refresh") {
            // semi's id, change if self-hosting
            if msg.author.id.to_string().eq("341813193464872991") {
                Command::new("git").arg("pull");
                let self = &Bot::with_layouts_in_dir("./layouts");
                send_message(&ctx, &msg, "Done :thumbsup:").await;
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
