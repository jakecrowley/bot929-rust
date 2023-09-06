use std::sync::Arc;
use poise::{Command, Framework, Context, FrameworkError};
use mongodm::prelude::{MongoDatabase, MongoCollection};

pub mod error;
use error::BotError;
use tokio::{sync::Mutex, runtime::Runtime};

use self::utils::FirstUser;

pub mod utils;

pub mod events;

#[derive(Clone, Debug)]
pub struct BotDatabase {
    pub database: MongoDatabase,
    pub archived: MongoCollection<crate::Nine92er>,
    pub nine29ers: MongoCollection<crate::Nine92er>,
    pub pastlist: MongoCollection<crate::Pastlist>,
    pub did929: Arc<Mutex<Vec<u64>>>,
    pub first: Arc<Mutex<FirstUser>>,
    pub runtime: Arc<Runtime>,
}

pub type BotContext<'a> = Context<'a, BotDatabase, BotError>;

pub type BotFramework = Framework<BotDatabase, BotError>;

pub type BotCommand = Command<BotDatabase, BotError>;

pub type BotResult<T> = Result<T, BotError>;

pub type BotFrameworkError<'a> = FrameworkError<'a, BotDatabase, BotError>;
