use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token = "Your_bot_token ";  // Replace with the your Bot Token
    let http = Http::new_with_token(&token);

    let guild_id = GuildId(705287712718061639); // Replace with the ID of your server

    let guild = guild_id.to_partial_guild(&http).await.expect("Error retrieving guild");
    println!("Number of users: {}", guild.approximate_member_count.unwrap_or(0));
}
