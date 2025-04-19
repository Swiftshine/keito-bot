use crate::context::*;
use anyhow::Result;
use crate::ppc;

/// Assemble a PowerPC instruction.
#[poise::command(slash_command)]
pub async fn assemble(
    ctx: Context<'_>,
    #[description = "The instruction to assemble."] instruction: String
) -> Result<()> {
    let code = ppc::instruction_to_code(&instruction)?;

    let reply_contents = format!("`0x{code:8X}`");

    let reply = create_reply(reply_contents);

    ctx.send(reply).await?;

    Ok(())
}
