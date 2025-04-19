use anyhow::Error;
use poise;

// user data
pub struct Data;
pub type Context<'a> = poise::Context<'a, Data, Error>;
