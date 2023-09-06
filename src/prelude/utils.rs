use std::{fmt::Debug, cmp::Ordering, thread };

use bson::{Bson, to_bson, doc};
use chrono::{Local, Timelike, DateTime };
use chrono_tz::US::Eastern;
use mongodm::{prelude::MongoCursor, f};
use poise::serenity_prelude::{Member, Http, Message, CreateMessage, Context, CacheHttp};
use serde::Serialize;
use poise::futures_util::TryStreamExt;
use serde_json::Value;

use crate::{Nine92er, Pastlist};

use super::{BotResult, error::{BsonSerSnafu, BotError}, BotDatabase};

#[derive(Debug)]
pub struct FirstUser {
    pub uid: u64,
    pub ts: i64,
}

pub struct TriggerTime {
    pub hour: u32,
    pub min: u32,
}

pub struct ChannelConfig {
    pub channel_id: u64,
    pub guild_id: u64,
}

static TRIGGER_TIME: TriggerTime = TriggerTime { hour: 9, min: 29 };
pub static CHANNEL_CONF: ChannelConfig = ChannelConfig { channel_id: 619704668590833692, guild_id: 377637608848883723 };

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

pub fn msg_to_json(msg: String) -> Value {
    return serde_json::to_value(&CreateMessage::default().content(msg).0).unwrap();
}

pub async fn nine29thread(ctx: &Context, data: &BotDatabase) {
    log::info!("Check 929 thread started!");

    loop {
        thread::sleep(std::time::Duration::from_millis(100));
        let mut now: DateTime<_> = Local::now().with_timezone(&Eastern);
        if (now.hour() == TRIGGER_TIME.hour || now.hour() == TRIGGER_TIME.hour+12) && (now.minute() == TRIGGER_TIME.min) {
            log::info!("It is 929!");
            
            while (now.hour() == TRIGGER_TIME.hour || now.hour() == TRIGGER_TIME.hour+12) && (now.minute() == TRIGGER_TIME.min)
            {
                now = Local::now().with_timezone(&Eastern);
                thread::sleep(std::time::Duration::from_millis(100));
            }

            log::info!("It is no longer 929!");

            thread::sleep(std::time::Duration::from_secs(1));

            let first: &mut FirstUser = &mut *data.first.lock().await;

            if first.uid == 0 {
                // let _ = ctx.send_message(CHANNEL_CONF.channel_id, &msg_to_json("Nobody did 929 :(".to_string())).await;
                log::info!("Nobody did 929 :(");
            } else {
                let firstuser = ctx.http().get_member(CHANNEL_CONF.guild_id, first.uid).await.unwrap();
                // let _ = ctx.send_message(CHANNEL_CONF.channel_id, &msg_to_json(
                //     format!("{} was first!", sanitize_username(firstuser.display_name().to_string()))
                // )).await;
                log::info!("{} was first!", sanitize_username(firstuser.display_name().to_string()));
            }

            let did929: &mut Vec<u64> = &mut *data.did929.lock().await;
            
            let _ = data.pastlist.delete_many(doc! {}, None).await;
            let mut docs: Vec<Pastlist> = Vec::new();
            for uid in did929.clone() {
                docs.push(Pastlist { _id: uid as i64, _t: "pastuser".to_string() })
            }
            let _ = data.pastlist.insert_many(docs, None).await;

            did929.clear();
            first.uid = 0;
        }
    }
}

pub async fn check_message_for_929(message: &Message, data: &BotDatabase) -> BotResult<()> {
    if message.channel_id.0 != CHANNEL_CONF.channel_id {
        return Ok(())
    }

    let msg: String = message.content.to_lowercase();
    let epoch: i64 = message.timestamp.timestamp_millis();
    let ts: DateTime<_> = message.timestamp.with_timezone(&Eastern);
    let author_id: u64 = message.author.id.0;

    if (ts.hour() == TRIGGER_TIME.hour || ts.hour() == TRIGGER_TIME.hour+12) && ts.minute() == TRIGGER_TIME.min {
        let did929: &mut Vec<u64> = &mut *data.did929.lock().await;
        let first: &mut FirstUser = &mut *data.first.lock().await;
        if msg.contains("929") && !did929.contains(&author_id) {
            did929.push(author_id);
            log::info!("{} did 929!", message.author.name);

            let author_id_bson: Bson = Bson::Int64(author_id as i64);
            let pl: Option<Pastlist> = data.pastlist.find_one(doc! { f!(_id in Pastlist): author_id_bson.clone() }, None).await?;
            let find: Option<Nine92er> = data.nine29ers.find_one(doc! { f!(_id in Nine92er): author_id_bson.clone() }, None).await?;

            let mut profile: Nine92er;
            match find {
                Some(n29er) => {
                    profile = n29er;
                },
                None => {
                    profile = Nine92er {
                        _id: author_id as i64,
                        currentstreak: 0,
                        points: 0.0,
                        maxstreak: 0,
                        count: 0,
                    };
                    data.nine29ers.insert_one(&profile, None).await?;
                }
            }

            if !pl.is_none() {
                profile.currentstreak += 1;
            } else {
                profile.currentstreak = 1;
            }

            if first.uid == 0 || epoch < first.ts {
                first.uid = author_id;
                first.ts = epoch;
                profile.points += 1.5 * ((1 + (profile.currentstreak / 5)) as f64);
            } else {
                profile.points += (1 + (profile.currentstreak / 5)) as f64;
            }
            
            if profile.currentstreak > profile.maxstreak {
                profile.maxstreak = profile.currentstreak;
            }

            profile.count += 1;

            let update = doc! { "$set": { "currentstreak": profile.currentstreak, "points": profile.points, "count": profile.count, "maxstreak": profile.maxstreak }};
            let update_result = data.nine29ers.update_one(doc! { f!(_id in Nine92er): author_id_bson }, update, None).await?;
            if update_result.modified_count != 1 {
                log::error!("Failed to update user {}", author_id);
            }
        }
    } else if msg.contains("929") {
        log::info!("User sent 929 but it is not 929 {}:{}", ts.hour(), ts.minute());
    }

    Ok(())
}

pub fn sanitize_username(str: String) -> String {
    return str.replace("||", "\\|\\|");
}

pub async fn get_leaderboard(data: &BotDatabase, http: &Http, position: usize) -> BotResult<String> {
    let mut leaderboard_str: String = "".to_owned();
    let members: Vec<Member> = http.get_guild_members(CHANNEL_CONF.guild_id, None, None).await?;

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
                log::warn!("Couldn't find member with ID {}", nine92er._id);
                users.remove(i);
                stop = if users.len() >= position + 10 { position + 10 } else { users.len() };
                continue;
            }
        }
        i += 1;
    }

    Ok(leaderboard_str)
}