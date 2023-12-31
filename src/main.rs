use std::env;
use std::sync::Arc;

use poise::{serenity_prelude as serenity, FrameworkOptions, PrefixFrameworkOptions};

use prelude::utils::FirstUser;
use prelude::{BotResult, BotDatabase};
use serde::{Serialize, Deserialize};
use serenity::{GatewayIntents, Context};

use mongodm::prelude::{MongoClientOptions, MongoCollection};
use mongodm::{mongo::options::ResolverConfig, prelude::MongoDatabase, prelude::MongoClient};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub mod prelude;
use crate::prelude::BotFramework;
use crate::prelude::events::event_handler;

pub mod commands;


use anyhow::Result as AnyResult;

pub struct BotConfig {
    pub bot_token: String,
    pub mongo_uri: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Nine92er {
    pub _id: i64,
    pub currentstreak: i32,
    pub points: f64,
    pub maxstreak: i32,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pastlist {
    pub _id: i64,
    pub _t: String,
}

#[tokio::main]
async fn main() -> AnyResult<()>{
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token: String = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");
    let mongo_uri: String = env::var("MONGO_URI").expect("Expected a MONGO_URI environment variable");
    let database: String = env::var("MONGO_DATABASE").expect("Expected a MONGO_DATABASE environment variable");

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
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES)
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx: &Context, event, framework, data| Box::pin(event_handler(ctx, event, framework, data)),
            commands: vec![commands::bot::profile(), commands::bot::leaderboard()],
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
    let database: MongoDatabase = setup_database(&config.mongo_uri, &config.database).await?;
    let col: MongoCollection<Nine92er> = database.collection("nine29ers");
    let archived: MongoCollection<Nine92er> = database.collection("archived_nine29ers");
    let pastlist_col: MongoCollection<Pastlist> = database.collection("pastlist");

    let db: BotDatabase = BotDatabase {
        database: database,
        archived: archived,
        nine29ers: col,
        pastlist: pastlist_col,
        did929: Arc::new(Mutex::new(Vec::new())),
        first: Arc::new(Mutex::new(FirstUser { uid: 0, ts: 0 })),
        runtime: Arc::new(Runtime::new().unwrap())
    };

    Ok(db)
}

async fn setup_database(uri: &str, database: &str) -> BotResult<MongoDatabase> {
    let client_options: MongoClientOptions = MongoClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
    let client: MongoClient = MongoClient::with_options(client_options)?;
    Ok(client.database(database))
}
