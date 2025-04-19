use anyhow::Error;
use poise;

// user data
pub struct Data;
pub type Context<'a> = poise::Context<'a, Data, Error>;


/// A helper function to create a non-pinging reply visible only
/// to the user of a slash command.
pub fn create_reply(reply_contents: String) -> poise::CreateReply {
    poise::CreateReply::default()
        .content(reply_contents)
        .ephemeral(true)
}
