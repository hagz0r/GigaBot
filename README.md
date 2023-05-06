# GigaBot
Sber Gigachat AI now in Telegram. Written in Rust

# Usage:
if you want to use it on your own machine somehow lol

- Create your own bot in @BotFather.
- Copy Token of your bot.
- Paste it in AppSettings.toml
- [Install mongoDB](https://www.mongodb.com/docs/manual/installation/)
- Copy your connection string 
- Paste it in AppSettings.toml

i used `gigachat_rust` as Main database name, and `teloxide` for users collection. 
create same structure or create your own, but don't forget to change names in code.

- [Install cargo, Rust env. etc.](https://www.rust-lang.org/tools/install)
- ```cargo run```

All settings are saved in AppSettings.toml
