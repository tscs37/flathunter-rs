#![feature(try_trait)]
#![feature(generator_trait)]
#![recursion_limit="128"]

mod errors;
pub use errors::{Error, ErrorKind, Result};
mod crawlers;
mod notifiers;
mod time_serializer;
use crawlers::Housing;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::str::FromStr;
pub use log::{trace,debug,info,warn,error};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeenData {
  first_timestamp: u64,
  last_timestamp: u64,
  first_data: Option<Housing>,
  last_data: Option<Housing>,
}

impl Default for SeenData {
  fn default() -> Self {
    SeenData{
      first_timestamp: 0,
      last_timestamp: 0,
      first_data: None,
      last_data: None,
    }
  }
}

fn default_evict_duration() -> std::time::Duration {
  // 1 Year
  humantime::parse_duration("1 year").unwrap()
}

fn default_run_duration() -> std::time::Duration {
  // 3 Hours
  humantime::parse_duration("3 hours").unwrap()
}

fn default_log_level() -> String {
  "debug".to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  #[serde(default = "default_evict_duration", with="time_serializer")]
  evict_after: std::time::Duration,
  #[serde(default = "default_run_duration", with="time_serializer")]
  run_every: std::time::Duration,
  #[serde(default)]
  notify_conf: BTreeMap<String, BTreeMap<String, String>>,
  #[serde(default = "default_log_level")]
  log_level: String,
  #[serde(default)]
  urls: Vec<String>,
  #[serde(default)]
  seen: BTreeMap<String, SeenData>,
}

fn setup_level_logger(level: log::LevelFilter) -> Result<()> {
  Ok(fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{} [{}@{}] {}",
        chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"),
        record.target(),
        record.level(),
        message,
      ))
    })
    .level(level)
    .level_for("html5ever", log::LevelFilter::Error)
    .level_for("hyper", log::LevelFilter::Error)
    .level_for("tokio_reactor", log::LevelFilter::Error)
    .level_for("reqwest", log::LevelFilter::Error)
    .chain(std::io::stderr())
    .apply()?)
}

fn setup_logger(config: &Config) -> Result<()> {
  Ok(setup_level_logger(log::LevelFilter::from_str(&config.log_level)?)?)
}

fn get_config() -> Result<Config> {
  match std::fs::metadata("flathunter.yml") {
    Err(_) => return Err(ErrorKind::NoConfigFound.into()),
    _ => (),
  }
  let config = std::fs::read("flathunter.yml")?;
  let config = std::string::String::from_utf8(config)?;
  let config: Config = serde_yaml::from_str(&config)?;
  Ok(config)
}

use crate::notifiers::{Notifier, NotifierContainer};

fn run(notif: Box<NotifierContainer>) -> Result<Config> {
  debug!("Loading configuration");
  let mut config = get_config()?;
  let mut results = Vec::new();
  info!("Loading fresh results");
  for url in &config.urls {
    debug!("Crawling {}", url);
    let crawler = crawlers::get_crawler(url.clone())?;
    let mut new_results = crawler.crawl_url(url.clone())?;
    results.append(&mut new_results);
  }
  info!("Got {} results", results.len());
  let mut new_results = Vec::new();
  for result in results {
    trace!("Result {:?}", result);
    let time = std::time::SystemTime::now();
    let time = time.duration_since(std::time::UNIX_EPOCH)?.as_secs();
    if !config.seen.contains_key(&result.clone().id) {
      trace!("New result {}", result.clone().id);
      config.seen.insert(result.clone().id, SeenData{
        first_timestamp: time,
        last_timestamp: time,
        first_data: Some(result.clone()),
        last_data: Some(result.clone()),
      });
      new_results.push(result.clone());
      notif.new_result(&config, result.clone())?;
    } else {
      trace!("already seend result {}", result.clone().id);
      config.seen.entry(result.clone().id).and_modify(|f| {
        f.last_timestamp = time;
        f.last_data = Some(result.clone());
      });
    }
  }
  debug!("writing out configuration file");
  let config_out = serde_yaml::to_string(&config)?;
  std::fs::write("flathunter.yml", config_out)?;
  Ok(config)
}

fn on_sig() {
  std::process::exit(128 + signal_hook::SIGINT);
}

fn main() -> Result<()> {
  human_panic::setup_panic!();
  debug!("Loading configuration");
  let mut config = get_config()?;
  debug!("Configuration parsed, applying to logger");
  setup_logger(&config)?;
  debug!("registering signal handler");
  unsafe {
    signal_hook::register(signal_hook::SIGINT, || on_sig())
  }.expect("could not setup sigint signal handler");
  unsafe {
    signal_hook::register(signal_hook::SIGTERM, || on_sig())
  }.expect("could not setup sigterm signal handler");
  debug!("Getting notifiers");
  let mut notif: Box<NotifierContainer> = notifiers::NotifierContainer::all_notifiers();
  notif.bootup_ok(&config)?;
  loop {
    notif = notifiers::NotifierContainer::all_notifiers();
    config = match run(notif) {
      Ok(v) => v,
      Err(v) => {
        notif = notifiers::NotifierContainer::all_notifiers();
        notif.error(&config, Box::new(v))?;
        config.clone()
      },
    };
    info!("sleeping for {} secs", config.run_every.as_secs());
    std::thread::sleep(config.run_every);
  }
}
