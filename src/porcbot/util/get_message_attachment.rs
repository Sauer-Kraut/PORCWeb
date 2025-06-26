use serenity::all::Message;
use reqwest::Client;
use crate::liberary::dialogue_lib::bot_error::BotError;

pub async fn get_message_attachment(msg: &Message, attachment_name_end: &str) -> Result<Vec<Vec<u8>>, BotError> {
    
    let mut attachments = vec!();

    for attachment in msg.attachments.iter() {

        if !attachment.filename.ends_with(attachment_name_end) {
            break;
        }

        let url = &attachment.url;
        let bytes = download_file(url).await?;

        attachments.push(bytes);
    }

    Ok(attachments)
}

async fn download_file(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>>{
    let client = Client::new();
    let req = client.get(url).send().await?;
    let bytes = req.bytes().await?;

    Ok(bytes.to_vec())
}