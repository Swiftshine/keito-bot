use crate::context::*;
use anyhow::Result;
use poise::serenity_prelude::{self as serenity, CreateAttachment};
use gfarch::gfarch;
use std::io::{Cursor, Write};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

/// Unpacks a GfArch archive and returns a zipped folder of its contents.
#[poise::command(slash_command)]
pub async fn gfarch_unpack(
    ctx: Context<'_>,
    #[description = "The GfArch archive to unpack"] attachment: serenity::Attachment
) -> Result<()> {
    // extract archive
    let archive_contents = attachment.download().await?;
    
    let extracted = gfarch::extract(&archive_contents)?;
    
    // create zip file
    let mut buffer = Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(&mut buffer);
    

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    for file in extracted {
        zip_writer.start_file(file.filename, options)?;
        zip_writer.write_all(&file.contents)?;
    }

    zip_writer.finish()?;

    // create attachment and reply to user
    let zip = buffer.into_inner();
    let zip_name = attachment.filename + ".zip";

    let output_attachment = CreateAttachment::bytes(zip, zip_name);

    let reply = poise::CreateReply::default()
        .attachment(output_attachment)
        .ephemeral(true); // only the user who sent the command can see the reply

    ctx.send(reply).await?;

    Ok(())
}
