use std::error::Error;

use rustc_hash::FxHashMap;
use teloxide::prelude::{Bot, ChatId, Requester};

pub async fn totg(msg: &str, map_info: &FxHashMap<String, String>) -> Result<(), Box<dyn Error>> {
    Bot::new(&map_info["api_key"])
        .send_message(ChatId(map_info["chat_id"].parse()?), msg)
        .await?;
    Ok(())
}

pub async fn send_to_other_api(
    msg: &str,
    maps_info: &[FxHashMap<String, String>],
) -> Result<(), Box<dyn Error>> {
    for el in maps_info.iter() {
        match el["key"].as_str() {
            "tg" => totg(msg, el).await?,
            _ => {}
        }
    }
    Ok(())
}
