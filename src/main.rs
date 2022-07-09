mod commands;
mod hooks;
mod handler;

use std::{collections::{HashSet, HashMap}, env, sync::{Arc, atomic::{AtomicUsize}}};
use commands::{shots::*};
use hooks::{counter, counter::{MessageCount, CommandCounter}};
use handler::Handler;
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{StandardFramework},
    http::Http,
    prelude::*,
};
use tracing::{error};


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
     let token = env::var("SHOT_BOT_TOKEN").expect("Expected a token in the environment");

    tracing_subscriber::fmt::init();

    // let http = Http::new(&token);

    // let (owners, _bot_id) = match http.get_current_application_info().await {
    //     Ok(info) => {
    //         let mut owners = HashSet::new();
    //         owners.insert(info.owner.id);

    //         (owners, info.id)
    //     }
    //     Err(why) => panic!("Could not access application info: {:?}", why),
    // };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        // .configure(|c| c.with_whitespace(false).prefix("!"))
        // .configure(|c| c.owners(owners).prefix("!"))
        // .before(counter::before)
        .group(&SHOTS_GROUP);

    let intents = GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error setting CTRL-C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}