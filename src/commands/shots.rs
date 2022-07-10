use crate::AdapterContainer;
use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use tracing::{error, info};

#[group]
#[commands(give, take, leaderboard, ponyup)]
pub struct Shots;

#[command]
#[description("Allocates a shot to the provided user")]
#[usage("[USER]")]
async fn give(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shot_saver = data.get::<AdapterContainer>().unwrap();

    let name: &str = msg.content.split(" ").collect::<Vec<&str>>()[1];
    let to_take = match shot_saver.add(name, 1).await {
        Ok(a) => a,
        Err(_) => -1,
    };
    let emoji = shot_emoji();
    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!(
                "{}  {} needs to take a shot!  {}",
                emoji, name, emoji
            ))
            .embed(|e| e.description(format!("They have {} shots left", &to_take)))
        })
        .await;

    Ok(())
}

#[command]
#[usage("[USER]")]
#[description("Removes a shot from the provided user")]
async fn take(ctx: &Context, msg: &Message) -> CommandResult {
    info!("take");

    let data = ctx.data.read().await;
    let shot_saver = data.get::<AdapterContainer>().unwrap();

    let name: &str = msg.content.split(" ").collect::<Vec<&str>>()[1];

    let shots_left = match shot_saver.subtract(name, 1).await {
        Ok(_a) => _a,
        Err(_) => -1,
    };
    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("{} took a shot (supposedly...)", name))
                .embed(|e| e.description(format!("They have {} shots left", &shots_left)))
        })
        .await;

    Ok(())
}

#[command]
#[description("Shows current standing of allocated shots")]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shot_saver = data.get::<AdapterContainer>().unwrap();
    let board = shot_saver.list().await;

    let _msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Leaderboard");

                let leaders = &board
                    .iter()
                    .map(|(k, v)| format!("{}, {}", k, v))
                    .collect::<Vec<String>>();
                e.description(leaders.join("\n"))
            })
        })
        .await;

    info!("{:?}", board);
    Ok(())
}

#[command]
#[usage("[USER]")]
#[description("Removes all shots from the given user")]
async fn ponyup(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shot_saver = data.get::<AdapterContainer>().unwrap();

    let name: &str = msg.content.split(" ").collect::<Vec<&str>>()[1];
    match shot_saver.reset(name).await {
        Ok(()) => {
            info!("Debt was cleared!");
            let _msg = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| e.description(format!("{}'s debt was cleared!", name)))
                })
                .await;
        }
        Err(()) => error!("Something went wrong"),
    };

    Ok(())
}

fn shot_emoji() -> &'static str {
    let emojis = vec!["🍷", "🍸", "🍹", "🍺", "🍻", "🥂", "🥃"];
    let mut rng = rand::thread_rng();
    let emoji = emojis.get(rng.gen_range(0..emojis.len())).unwrap();
    emoji
}
