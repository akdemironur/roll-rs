use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
mod context;
mod roll;
mod save;
use crate::context::Data;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let data = Data {
        saved_rolls: Arc::new(Mutex::new(HashMap::new())),
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roll::r(), save::save(), save::view(), save::remove()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
