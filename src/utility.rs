use keynergy::layout::Layout;
use rust_fuzzy_search::fuzzy_search_best_n;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn send_message(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn closest_match(name: String, names: &[String]) -> String {
    let names = names.iter().map(String::as_ref).collect::<Vec<&str>>();
    fuzzy_search_best_n(&name, &names, 1)[0].0.to_string()
}

pub fn display_matrix(m: &[Vec<char>]) -> String {
    let mut s = String::new();
    for r in m.iter() {
        for (i, c) in r.iter().enumerate() {
            s.push(*c);
            s.push(' ');
            if i == 4 {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

pub fn print_layout(l: &Layout) -> String {
    let year = match l.year {
        0 => "".to_string(),
        _ => std::format!("({})", l.year),
    };
    let link = match &l.link {
        Some(x) => std::format!("<https://{}>\n", x),
        None => "".to_string(),
    };
    std::format!(
        "**{}**\nCreated by {} {}\n{}```\n{}\n```",
        l.name,
        l.author,
        year,
        link,
        display_matrix(&l.formats.standard.as_ref().unwrap().matrix)
    )
}
