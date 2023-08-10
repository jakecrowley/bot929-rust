use std::env;
use std::sync::Arc;

use poise::{serenity_prelude as serenity, FrameworkOptions, PrefixFrameworkOptions};

use prelude::{BotResult, BotDatabase};
use serenity::{GatewayIntents, Context};

use mongodm::prelude::MongoClientOptions;
use mongodm::{mongo::options::ResolverConfig, prelude::MongoDatabase, prelude::MongoClient};

pub mod prelude;
use crate::prelude::BotFramework;

pub mod commands;


use anyhow::Result as AnyResult;

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
        .intents(GatewayIntents::privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES)
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: vec![commands::bot::profile()],
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
