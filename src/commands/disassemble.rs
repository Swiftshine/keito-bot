use crate::context::*;
use anyhow::Result;
use crate::ppc;

/// Disassemble a PowerPC instruction.
#[poise::command(slash_command)]
pub async fn disassemble(
    ctx: Context<'_>,
    #[description = "The code of the instruction to disassemble (in hex)."] code: String
) -> Result<()> {
    let code = if let Some(hex) = code.strip_prefix("0x") {
        u32::from_str_radix(&hex, 16)?
    } else {
        u32::from_str_radix(&code, 16)?
    };

    let instruction = ppc::code_to_instruction(code);

    let reply = create_reply(instruction);

    ctx.send(reply).await?;

    Ok(())
}
