use poise::{Command, Framework};
use mongodm::prelude::MongoDatabase;

pub mod error;
use error::BotError;

#[derive(Clone, Debug)]
pub struct BotDatabase {
    pub database: MongoDatabase,
}

pub type BotFramework = Framework<BotDatabase, BotError>;

pub type BotCommand = Command<BotDatabase, BotError>;

pub type BotResult<T> = Result<T, BotError>;