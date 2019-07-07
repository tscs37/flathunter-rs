use crate::Housing;
use crate::Config;
use crate::Result;

mod webhook;

pub trait Notifier {
  fn new_result(&self, cfg: &Config, res: Housing) -> Result<()> {
    self.post_message(cfg,
      &format!("New result: {} rooms ({} m²) for {} €: \"{}\". Location: {}. Link: {}",
        res.rooms,
        res.size,
        res.price,
        res.title,
        res.address,
        res.url,
    ))?;
    Ok(())
  }
  fn post_message(&self, cfg: &Config, msg: &str) -> Result<()>;
  fn bootup_ok(&self, cfg: &Config) -> Result<()> {
    self.post_message(cfg, "Flathunter has booted and is scanning for flats")?;
    Ok(())
  }
  fn error(&self, cfg: &Config, e: Box<dyn std::error::Error>) -> Result<()> {
    self.post_message(cfg, &format!("Error in Flathunter: {}", e.description()))?;
    Ok(())
  }
}

#[derive(Clone)]
pub struct StdoutNotifier{}

impl Notifier for StdoutNotifier {
  fn post_message(&self, _cfg: &Config, m: &str) -> Result<()> {
    println!("{}", m);
    Ok(())
  }
}

pub struct NotifierContainer {
  notifiers: Vec<Box<dyn Notifier>>,
}

impl Notifier for NotifierContainer {
  fn new_result(&self, cfg: &Config, res: Housing) -> Result<()> {
    for notifier in &self.notifiers {
      notifier.new_result(cfg, res.clone())?;
    }
    Ok(())
  }
  fn post_message(&self, cfg: &Config, msg: &str) -> Result<()> {
    for notifier in &self.notifiers {
      notifier.post_message(cfg, msg)?;
    }
    Ok(())
  }
  fn bootup_ok(&self, cfg: &Config) -> Result<()> {
    for notifier in &self.notifiers {
      notifier.bootup_ok(cfg)?;
    }
    Ok(())
  }
}

impl NotifierContainer {
  pub fn all_notifiers() -> Box<NotifierContainer> {
    let notifiers: Vec<Box<dyn Notifier>> = vec![
      Box::new(StdoutNotifier{}),
      Box::new(webhook::WebhookNotifier{})
    ];
    Box::new(NotifierContainer {
      notifiers: notifiers,
    })
  }
}