const CONFIG_KEY: &str = "webhook";
use log::*;
use crate::{Result, Config};
use serde::Serialize;
use std::collections::BTreeMap;
use super::Notifier;
use crate::errors::ErrorKind;
use crate::Housing;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Embed {
  Fields(EmbedFields),
  URL(EmbedURL),
  Title(EmbedTitle),
  Description(EmbedDescription),
}

#[derive(Serialize)]
pub struct EmbedFields {
  fields: Vec<EmbedField>,
}


#[derive(Serialize)]
pub struct EmbedField {
  name: String,
  value: String,
  inline: bool,
}

#[derive(Serialize)]
pub struct EmbedURL {
  title: String,
  url: String,
}

#[derive(Serialize)]
pub struct EmbedTitle {
  title: String,
}

#[derive(Serialize)]
pub struct EmbedDescription {
  description: String
}

#[derive(Clone)]
pub struct WebhookNotifier{}

impl Notifier for WebhookNotifier {
  fn new_result(&self, cfg: &Config, res: Housing) -> Result<()> {
    let conf = cfg.notify_conf.get(CONFIG_KEY);
    let conf: &BTreeMap<String, String> = match conf {
      None => { return Ok(()); },
      Some(c) => c,
    };
    let hook_type: &String = conf.get("type")?;
    let hook_type = hook_type.to_lowercase();
    let hook_url = conf.get("url")?;
    debug!("sending new result webhook to '{}' ({})", hook_url, hook_type);
    match hook_type.as_str() {
      "discord" => {
        #[derive(Serialize)]
        struct Webhook {
          embeds: Vec<Embed>,
          username: Option<String>,
          avatar_url: Option<String>,
        };
        use Embed::*;
        let form = Webhook{
          embeds: vec![
            Title(EmbedTitle{ title: format!("{} ({} €, {} Rooms, {} m²). {}.",
              res.title,
              res.address,
              res.price,
              res.rooms,
              res.size,
            )}),
            Fields(EmbedFields{ fields: vec![
              EmbedField{ name: "Title".to_string(), value: format!("[{}]({})", res.title.clone(), res.url.clone()), inline: false },
              EmbedField{ name: "Address".to_string(), value: res.address.clone(), inline: true },
              EmbedField{ name: "Rooms".to_string(), value: format!("{}", res.rooms), inline: true },
              EmbedField{ name: "Size".to_string(), value: format!("{} m²", res.size), inline: true },
              EmbedField{ name: "Price".to_string(), value: format!("{} €", res.price), inline: true },
            ]}),
          ],
          username: conf.get("display_name").map(|y| y.clone()),
          avatar_url: conf.get("display_img").map(|y| y.clone()),
        };
        debug!("json post: {}", serde_json::to_string_pretty(&form)?);
        let client: reqwest::Client = reqwest::ClientBuilder::new().build()?;
        client.post(hook_url)
          .json(&form)
          .send()?;
      }
      _ => return Err(ErrorKind::UnknownWebhookType(hook_type.to_string()).into()),
    }
    Ok(())
  }
  fn post_message(&self, cfg: &Config, msg: &str) -> Result<()> {
    let conf = cfg.notify_conf.get(CONFIG_KEY);
    let conf: &BTreeMap<String, String> = match conf {
      None => { return Ok(()); },
      Some(c) => c,
    };
    let hook_type: &String = conf.get("type")?;
    let hook_type = hook_type.to_lowercase();
    let hook_url = conf.get("url")?;
    debug!("sending webhook to '{}' ({})", hook_url, hook_type);
    match hook_type.as_str() {
      "discord" => {
        #[derive(Serialize)]
        struct Webhook {
          content: String,
          username: Option<String>,
          avatar_url: Option<String>,
        };
        let form = Webhook{
          content: msg.to_string(),
          username: conf.get("display_name").map(|y| y.clone()),
          avatar_url: conf.get("display_img").map(|y| y.clone()),
        };
        let client: reqwest::Client = reqwest::ClientBuilder::new().build()?;
        client.post(hook_url)
          .form(&form)
          .send()?;
      }
      _ => return Err(ErrorKind::UnknownWebhookType(hook_type.to_string()).into()),
    }
    Ok(())
  }
}