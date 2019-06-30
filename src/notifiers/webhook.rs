const CONFIG_KEY: &str = "webhook";
use log::*;
use crate::{Result, Config};
use serde::Serialize;
use std::collections::BTreeMap;
use super::Notifier;
use crate::errors::ErrorKind;

#[derive(Clone)]
pub struct WebhookNotifier{}

impl Notifier for WebhookNotifier {
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