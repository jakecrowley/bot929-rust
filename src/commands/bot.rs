use bson::{doc, Bson};
use mongodm::f;

use mongodm::prelude::MongoCursor;
use poise::command;
use poise::futures_util::TryStreamExt;
use poise::serenity_prelude::{User, CreateEmbed, ChannelId, GuildId, CacheHttp};

use crate::prelude::{BotContext, BotResult};
use crate::Nine92er;

#[command(prefix_command)]
pub async fn profile(
    ctx: BotContext<'_>,
    #[description = "Retrieve your or someone else's 929 profile."] user: Option<User>,
) -> BotResult<()> {
    let channel_id: ChannelId = ctx.channel_id();

    if channel_id.0 != 674812650907238405 {
        return Ok(())
    }

    let user: &User = user.as_ref().unwrap_or(ctx.author());
    let guild_id: GuildId = ctx.guild_id().unwrap();

    let nickname: String = user.nick_in(&ctx.http(), guild_id).await.unwrap_or(user.name.clone());

    let user_id_bson: Bson = Bson::Int64(user.id.0 as i64);
    let profile: Option<Nine92er> = ctx.data().nine29ers.find_one(doc! { f!(_id in Nine92er): user_id_bson }, None).await?;
    
    match profile {
        Some(profile) => {
            let msg = ctx.send(|resp| 
                resp.embed(|e: &mut CreateEmbed| {
                    e.title(format!("Profile for {}", nickname))
                     .fields(vec![
                        ("Current Streak", profile.currentstreak, false),
                        ("Longest Streak", profile.maxstreak, false),
                     ])
                     .field("Total Points", profile.points, false)
                     .field("Total 929s", profile.count, false)
                }).reply(true)
            ).await;


            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        },
        None => {
            ctx.send(|response: &mut poise::CreateReply<'_>| response.content("You have not participated in a 9:29 yet!").reply(true)).await?;
        }
    };

    Ok(())
}

#[command(prefix_command)]
pub async fn leaderboard(ctx: BotContext<'_>) -> BotResult<()> {
    // let mut leaderboard_str: String = "".to_owned();

    let mut users: Vec<Nine92er> = Vec::new();

    let mut users_cursor: MongoCursor<Nine92er> = ctx.data().nine29ers.find(None, None).await?;
    while let Some(nine92er) = users_cursor.try_next().await?{
        users.push(nine92er);
    }
    users.sort_by(|a, b| a.count.cmp(&b.count).reverse());

    for user in users {
        println!("{}", user._id)
    }

    // let position: i32 = 0;
    // let stop: i32 = 10;

    Ok(())
}