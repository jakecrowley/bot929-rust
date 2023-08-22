use poise::{Command, Framework, Context, FrameworkError};
use mongodm::prelude::{MongoDatabase, MongoCollection};

pub mod error;
use error::BotError;

pub mod utils;

pub mod events;

#[derive(Clone, Debug)]
pub struct BotDatabase {
    pub database: MongoDatabase,
    pub nine29ers: MongoCollection<crate::Nine92er>,
}

pub type BotContext<'a> = Context<'a, BotDatabase, BotError>;

pub type BotFramework = Framework<BotDatabase, BotError>;

pub type BotCommand = Command<BotDatabase, BotError>;

pub type BotResult<T> = Result<T, BotError>;

pub type BotFrameworkError<'a> = FrameworkError<'a, BotDatabase, BotError>;
