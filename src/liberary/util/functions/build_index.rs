use crate::backend::backend_api::server_error::ServerError;


pub fn build_index(json_contents: String, body: String, argument_key: &str) -> Result<String, ServerError> {
    
    let mut filled_json = String::new();
    let mut parts = body.split(&argument_key).collect::<Vec<&str>>();
    
    if parts.len() < 2 {
        return Err("Could not find position to inject preload data into index string".into());
    }
    else {
        filled_json.push_str(parts.first().unwrap());
        filled_json.push_str(&json_contents);
        for part in parts.split_off(1).iter() {
            filled_json.push_str(part);
        }
        Ok(filled_json)
    }
}