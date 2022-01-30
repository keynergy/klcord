# klcord
A basic Discord bot for getting keyboard layout information, powered by the [Keynergy](https://github.com/keynergy/keynergy-lib) library.
# Usage
![!layout](./klcord_example.png)
# Add to your servers
Use [this link](https://discord.com/api/oauth2/authorize?client_id=861041190329778246&permissions=2048&scope=bot) to add the bot to your server!
# Contributing
## Adding or editing layouts
- If you're adding a new layout, duplicate an existing one to use as reference.
- If there is no website for the layout, or you don't know it, you can either set `link = ""` or just get rid of the whole line. If the layout has a website, but it isn't in the bot, please add it!
- If you don't know what year the layout was released, just set it to 0. If you *do* know the release year, but it's not there, please add it! 
- Keep `map` as `{}`, it won't work without this.
- If the layout has more than three rows, set `home_row` to the index where the home row is, starting at 0. This isn't important for the bot right now, but it's good future-proofing.
- If the layout has a thumb row, set `thumb_row` to the index where the thumb row is, starting at 0. 
## Code
KLCord has a very simple codebase right now. Even if you're not great at Rust yet, it shouldn't be very hard to work with! Take a look at the Issues if you want to work on something. You might want a basic familiarization with [Keynergy](https://github.com/keynergy/keynergy-lib) and [Serenity](https://github.com/serenity-rs/serenity).
