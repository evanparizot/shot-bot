use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use tracing::info;
use crate::{ShotSaver, AdapterContainer};

#[group]
#[commands(give, take)]
pub struct Shots;

#[command]
async fn give(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.read().await;
    let shot_saver = data.get::<AdapterContainer>().unwrap();

    shot_saver.add("", 1).await;

    let _msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Needs to take a shot!")
    }).await;

    Ok(())
}


#[command]
async fn take(ctx: &Context, msg: &Message) -> CommandResult {
    info!("take");
    Ok(())
}

