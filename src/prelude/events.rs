use poise::serenity_prelude::{CurrentUser, MessageComponentInteraction, Context, Embed, CreateEmbed, Color, ButtonStyle, ReactionType};
use poise::serenity_prelude::CacheHttp;
use poise::Event;

use crate::prelude::utils::{get_leaderboard, nine29thread};

use super::utils::check_message_for_929;
use super::{BotDatabase, error::BotError, BotResult};

pub async fn event_handler(ctx: &Context, event: &Event<'_>, _framework: poise::FrameworkContext<'_, BotDatabase, BotError>, data: &BotDatabase) -> BotResult<()>{
    match event {
        Event::Ready { data_about_bot } => {
            let user: &CurrentUser = &data_about_bot.user;
            let ctx_clone = ctx.clone();
            let data_clone = data.clone();

            //let rt = Runtime::new().unwrap();
            data.runtime.spawn(async move {
                nine29thread(&ctx_clone, &data_clone).await;
            });

            println!("Bot ready! Logged in as user: {}#{}", user.name, user.discriminator);
        }
        Event::Message { new_message } => {
            let res = check_message_for_929(new_message, data).await;
            match res {
                Err(e) => {
                    println!("{}", e.to_string())
                },
                Ok(_) => {}
            }
        }
        Event::InteractionCreate { interaction } => {
            let mut comp: MessageComponentInteraction = interaction.clone().into_message_component().expect("Interaction could not be converted into message component.");
            println!("Component interaction triggered: custom_id = {}", comp.data.custom_id);

            match comp.data.custom_id.as_str() {
                "prev" => {
                    let embed: &Embed = comp.message.embeds.get(0).expect("Message does not contain an embed!");
                    let first_pos: usize = embed.description.as_ref().expect("no description").split_once(".").unwrap().0.to_string().parse::<usize>().unwrap();
                    if first_pos >= 10 {
                        let leaderboard_str = get_leaderboard(data, &ctx.http, first_pos - 11).await?;
                        comp.message.edit(ctx.http(), |resp| 
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
                            })
                        ).await?;
                    }
                    comp.defer(ctx.http()).await?;
                },
                "next" => {
                    let embed: &Embed = comp.message.embeds.get(0).expect("Message does not contain an embed!");
                    let first_pos: String = embed.description.as_ref().expect("no description").split_once(".").expect("no dot").0.to_string();
                    let leaderboard_str_res = get_leaderboard(data, &ctx.http, first_pos.parse::<usize>().unwrap() + 9).await;
                    let leaderboard_str: String = match leaderboard_str_res {
                        Ok(ldb) => ldb,
                        Err(err) => {
                            println!("{}", err);
                            comp.defer(ctx.http()).await?;
                            return Ok(());
                        }
                    };

                    comp.message.edit(ctx.http(), |resp: &mut poise::serenity_prelude::EditMessage<'_>| 
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
                        })
                    ).await?;
                    comp.defer(ctx.http()).await?;
                }
                _ => {},
            }
        }
        _ => {}
    }

    Ok(())
}