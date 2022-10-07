use std::collections::HashMap;
use std::fs;

use keynergy::layout::Layout;
use rust_fuzzy_search::fuzzy_search_best_n;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn send_message(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn get_layouts_from_dir(dir: &str) -> HashMap<String, Layout> {
    let dir = fs::read_dir(format!("./{}", dir)).unwrap();

    let mut layouts: HashMap<String, Layout> = HashMap::new();
    for file in dir.flatten() {
	if let Some(path) = file.path().to_str() {
	    match Layout::load(path) {
		Ok(mut l) => {
		    if l.link == Some(String::from("")) {
			l.link = None;
		    }
		    let name = l.name.to_ascii_lowercase();
		    layouts.insert(name, l);
		}
		Err(e) => {
		    println!("{}: {:?}", path, e);
		}
	    }
	}
    }
    layouts
}

pub fn closest_match(name: String, names: &[&str]) -> String {
    fuzzy_search_best_n(&name, names, 1)[0].0.to_string()
}

pub fn display_matrix(m: &[Vec<char>], angle: bool) -> String {
    let mut s = String::new();
    for (y, r) in m.iter().enumerate() {
	if angle && y == 2 {
	    s.push(' ');
	}
        for (x, c) in r.iter().enumerate() {
	    s.push(*c);
	    s.push(' ');
	    if x == 4 {
                s.push(' ');
	    }
        }
        s.push('\n');
    }
    s
}

pub fn print_layout(l: &Layout) -> String {
    let angle = l.angle_is_preferred();
    let year = match l.year {
        0 => "".to_string(),
        _ => std::format!("({})", l.year),
    };
    let link = match &l.link {
        Some(x) => std::format!("<https://{}>\n", x),
        None => "".to_string(),
    };
    let keys = match angle {
        true => l.formats.angle.as_ref(),
        false => l.formats.standard.as_ref(),
    };
    std::format!(
        "**{}** {}\nCreated by {} {}\n{}\nMade for {}```\n{}\n```",
        l.name,
        if angle { "with angle mod" } else { "" },
        l.author,
        year,
        link,
        l.language,
        display_matrix(&keys.unwrap().matrix, angle)
    )
}
