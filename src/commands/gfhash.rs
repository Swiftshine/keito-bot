use crate::context::*;
use anyhow::Result;

/// Generates a 32-bit Good-Feel hash from a string input.
#[poise::command(slash_command)]
pub async fn gfhash(
    ctx: Context<'_>,
    #[description = "The string input."] input: String,
    #[description = "Display in decimal format?"] display_decimal: Option<bool>
) -> Result<()> {

    let mut result = 0u32;
    
    for c in input.bytes() {
        result = c as u32 + result.wrapping_mul(137);
    }

    let reply_contents = if display_decimal.is_none() {
        format!("`0x{result:8X}`")
    } else {
        if display_decimal.unwrap() {
            format!("`0x{result:8X}`")
        } else {
            format!("`{result}`")
        }
    } + &format!(" (`input`)");

    ctx.reply(reply_contents).await?;

    Ok(())
}
