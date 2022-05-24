use rand::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if !msg.author.bot {
      if let Some(guild) = msg.guild_id {
        if let Ok(emojis) = guild.emojis(&ctx.http).await {
          let e = {
            let mut rng = rand::thread_rng();
            emojis.into_iter().choose(&mut rng).unwrap()
          };
          if let Err(why) = msg.channel_id.say(&ctx.http, e).await {
            println!("Error sending message: {:?}", why);
          }
        }
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
