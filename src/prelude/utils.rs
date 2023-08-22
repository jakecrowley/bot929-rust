use std::{fmt::Debug, cmp::Ordering};

use bson::{Bson, to_bson};
use mongodm::prelude::MongoCursor;
use poise::serenity_prelude::{ Member, Http };
use serde::Serialize;
use poise::futures_util::TryStreamExt;

use crate::Nine92er;

use super::{BotResult, error::{BsonSerSnafu, BotError}, BotDatabase};

// fn get_type<T>(_: &T) -> BotResult<String> {
//     Ok(std::any::type_name::<T>().to_string())
// }

/// Extension trait for converting any [`Serialize`]-able type to BSON through a method.
pub trait SerializeExt: Serialize {
    /// Converts the value to [`Bson`].
    fn to_bson(&self) -> BotResult<Bson>;
}

impl<T> SerializeExt for T
where
    T: Serialize + Debug,
{
    fn to_bson(&self) -> BotResult<Bson> {
        snafu::ResultExt::with_context(to_bson(self), |_| BsonSerSnafu {
            debug: format!("{:?}", self),
        })
    }
}

pub fn sanitize_username(str: String) -> String {
    return str.replace("||", "\\|\\|");
}

pub async fn get_leaderboard(data: &BotDatabase, http: &Http, position: usize) -> BotResult<String> {
    let mut leaderboard_str: String = "".to_owned();
    let members: Vec<Member> = http.get_guild_members(377637608848883723, None, None).await?;
    // let guild = ctx.guild_id().expect("Unable to unwrap guild id");

    let mut users: Vec<Nine92er> = Vec::new();

    let mut users_cursor: MongoCursor<Nine92er> = data.nine29ers.find(None, None).await?;
    while let Some(nine92er) = users_cursor.try_next().await?{
        users.push(nine92er);
    }
    users.sort_by(|a, b| a.points.partial_cmp(&b.points).unwrap_or(Ordering::Equal));
    users.reverse();

    if position > users.len() {
        return Err(BotError::Generic { message: "Position exceeds user list length" });
    }

    let mut stop: usize = if users.len() >= position + 10 { position + 10 } else { users.len() };

    let mut i: usize = position;
    while i < stop {
        let nine92er: &Nine92er = users.get(i).unwrap();
        let member: Option<Member> = members.clone().into_iter().find(|r| r.user.id.0 == nine92er._id as u64);

        match member {
            Some(m) => {
                leaderboard_str.push_str(format!("{}. {}: {}\n", i+1, sanitize_username(m.display_name().to_string()), nine92er.points).as_str());
            }
            None => {
                println!("Couldn't find member with ID {}", nine92er._id);
                users.remove(i);
                stop = if users.len() >= position + 10 { position + 10 } else { users.len() };
                continue;
            }
        }
        i += 1;
    }

    Ok(leaderboard_str)
}