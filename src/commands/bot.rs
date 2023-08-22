use std::cmp::Ordering;

use bson::{doc, Bson};
use mongodm::f;

use mongodm::prelude::MongoCursor;
use poise::command;
use poise::futures_util::TryStreamExt;
use poise::serenity_prelude::{User, CreateEmbed, GuildId, CacheHttp, Member, Color, ButtonStyle, ReactionType};

use crate::prelude::utils::sanitize_username;
use crate::prelude::{BotContext, BotResult};
use crate::Nine92er;

#[command(prefix_command)]
pub async fn profile(
    ctx: BotContext<'_>,
    #[description = "Retrieve your or someone else's 929 profile."] user: Option<User>,
) -> BotResult<()> {
    if ctx.channel_id().0 != 674812650907238405 {
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
    if ctx.channel_id().0 != 674812650907238405 {
        return Ok(())
    }

    let mut leaderboard_str: String = "".to_owned();
    let members: Vec<Member> = ctx.http().get_guild_members(377637608848883723, None, None).await?;
    // let guild = ctx.guild_id().expect("Unable to unwrap guild id");

    let mut users: Vec<Nine92er> = Vec::new();

    let mut users_cursor: MongoCursor<Nine92er> = ctx.data().nine29ers.find(None, None).await?;
    while let Some(nine92er) = users_cursor.try_next().await?{
        users.push(nine92er);
    }
    users.sort_by(|a, b| a.points.partial_cmp(&b.points).unwrap_or(Ordering::Equal));
    users.reverse();

    let position: usize = 0;
    let stop: usize = if users.len() >= position + 10 { position + 10 } else { users.len() };

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
                continue;
            }
        }
        i += 1;
    }

    let _msg = ctx.send(|resp| 
        resp.embed(|e: &mut CreateEmbed| {
            e.title("Leaderboard")
             .description(leaderboard_str)
             .color(Color::DARK_RED)
        }).components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.style(ButtonStyle::Primary).custom_id("prev").label("").emoji(ReactionType::Unicode("⬅️".to_string()))
                }).create_button(|b| {
                    b.style(ButtonStyle::Primary).custom_id("next").label("").emoji(ReactionType::Unicode("➡️".to_string()))
                })
            })
        }).reply(true)
    ).await?;

    Ok(())
}