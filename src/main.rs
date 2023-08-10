use std::env;
use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::{Client, async_trait};
use serenity::framework::StandardFramework;
use serenity::framework::standard::Configuration;
use serenity::http::Http;
use serenity::model::prelude::{UserId, Ready};
use serenity::prelude::{GatewayIntents, Context, EventHandler, TypeMapKey};
use tokio::sync::Mutex;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");

    let http = Http::new(&token);

    let bot_id: UserId = match http.get_current_user().await {
        Ok(bot_id) => bot_id.id,
        Err(why) => panic!("Could not access bot id: {:?}", why),
    };

    let framework: StandardFramework = StandardFramework::new()
        .configure(|c: &mut Configuration| c
                        .with_whitespace(true)
                        .on_mention(Some(bot_id))
                        .prefix("!")); 


    let intents: GatewayIntents = GatewayIntents::privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Error creating client");
    
    {
        let mut data: tokio::sync::RwLockWriteGuard<'_, serenity::prelude::TypeMap> = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

}


