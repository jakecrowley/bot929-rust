use poise::{Command, Framework, Context, FrameworkError};
use mongodm::prelude::MongoDatabase;

pub mod error;
use error::BotError;

#[derive(Clone, Debug)]
pub struct BotDatabase {
    pub database: MongoDatabase,
}

pub type BotContext<'a> = Context<'a, BotDatabase, BotError>;

pub type BotFramework = Framework<BotDatabase, BotError>;

pub type BotCommand = Command<BotDatabase, BotError>;

pub type BotResult<T> = Result<T, BotError>;

pub type BotFrameworkError<'a> = FrameworkError<'a, BotDatabase, BotError>;
