mod context;
mod commands;
mod ppc;

use dotenvy;
use anyhow::Result;
use context::*;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<()> {
    // try to load env vars from .env
    dotenvy::dotenv()?;

    let token = std::env::var("DISCORD_TOKEN").expect("Can't find DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::default();

    let framework_options = poise::FrameworkOptions {
        commands: vec![
            commands::gfhash::gfhash(),
            commands::gfarch_unpack::gfarch_unpack(),
            commands::assemble::assemble(),
            commands::disassemble::disassemble()
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                
                Ok(Data)
            })
        })
        .build();
    
    // create client
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();

    Ok(())
}
