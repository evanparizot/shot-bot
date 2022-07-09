use std::sync::atomic::Ordering;

use serenity::{async_trait, client::{EventHandler, Context}, model::{channel::Message, event::ResumedEvent, prelude::Ready}};
use tracing::info;

use crate::hooks::counter::MessageCount;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.content.to_lowercase().contains("owo") {
            let count = {
                let data_read = ctx.data.read().await;
                data_read.get::<MessageCount>().expect("bal").clone()
            };
            count.fetch_add(1, Ordering::SeqCst);
        }


        info!("{}", message.id);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}