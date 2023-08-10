use std::env;
use std::sync::Arc;

use poise::{serenity_prelude as serenity, FrameworkOptions};

use prelude::{BotResult, BotDatabase};
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::prelude::{UserId, Ready};
use serenity::{GatewayIntents, Context, EventHandler};

use mongodm::prelude::MongoClientOptions;
use mongodm::{mongo::options::ResolverConfig, prelude::MongoDatabase, prelude::MongoClient};

pub mod prelude;
use crate::prelude::BotFramework;

use anyhow::Result as AnyResult;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub struct BotConfig {
    pub bot_token: String,
    pub mongo_uri: String,
    pub database: String,
}

#[tokio::main]
async fn main() -> AnyResult<()>{
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token: String = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");
    let mongo_uri: String = env::var("MONGO_URI").expect("Expected a DISCORD_TOKEN environment variable");
    let database: String = env::var("MONGO_DATABASE").expect("Expected a DISCORD_TOKEN environment variable");

    // let http = Http::new(&token);

    // let bot_id: UserId = match http.get_current_user().await {
    //     Ok(bot_id) => bot_id.id,
    //     Err(why) => panic!("Could not access bot id: {:?}", why),
    // };

    let config: BotConfig = BotConfig {
        bot_token: token,
        mongo_uri: mongo_uri,
        database: database,
    };

    Bot::new(config).await?.start().await?;

    Ok(())
    
}

pub struct Bot(Arc<BotFramework>);

impl Bot {
    pub async fn new(config: BotConfig) -> BotResult<Self>{
        let framework = BotFramework::builder()
        .token(&config.bot_token)
        .intents(GatewayIntents::privileged() | GatewayIntents::MESSAGE_CONTENT)
        .options(FrameworkOptions {
            ..Default::default()
        })
        .setup(move |_context: &Context, _, _framework: &poise::Framework<BotDatabase, prelude::error::BotError>| {
            Box::pin(setup_bot_database(config))
        })
        .build().await?;

        Ok(Self(framework))
    }

    pub async fn start(self) -> BotResult<()> {
        self.0.start().await?;
        Ok(())
    }
}

// async fn setup_bot_database(context: &Context, framework: &BotFramework, mongo_uri: &str, mongo_db: &str) -> BotResult<BotDatabase>
async fn setup_bot_database(config: BotConfig) -> BotResult<BotDatabase>
{
    let db = BotDatabase {
        database: setup_database(&config.mongo_uri, &config.database).await?
    };

    // do actual other stuff idk

    Ok(db)
}

async fn setup_database(uri: &str, database: &str) -> BotResult<MongoDatabase> {
    let client_options = MongoClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
    let client = MongoClient::with_options(client_options)?;
    Ok(client.database(database))
}
