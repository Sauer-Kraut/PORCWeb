use serenity::all::Message;
use reqwest::Client;
use serde_json;

pub async fn get_message_attachment(msg: &Message, attachment_name_end: &str) -> Result<Vec<Vec<u8>>, String> {
    
    let mut attachments = vec!();

    for attachment in msg.attachments.iter() {

        if !attachment.filename.ends_with(attachment_name_end) {
            break;
        }

        let url = &attachment.url;
        let bytes = match download_file(url).await {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

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