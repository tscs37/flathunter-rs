use rust_decimal::Decimal;
use crate::Result;

use serde::{Serialize, Deserialize};

mod immoscout;
pub use immoscout::ImmoScout;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Housing {
  pub id: String,
  pub url: String,
  pub title: String,
  pub price: Decimal,
  pub size: Decimal,
  pub rooms: Decimal,
  pub address: String,
}

pub trait Crawler {
  fn match_url(&self, url: String) -> bool;
  fn crawl_url(&self, url: String) -> Result<Vec<Housing>>;
}

pub fn get_crawler(url: String) -> Option<Box<dyn Crawler>> {
  let crawlers: Vec<Box<dyn Crawler>> = vec![
    Box::new(ImmoScout{}),
  ];
  for crawler in crawlers {
    if crawler.match_url(url.clone()) {
      return Some(crawler);
    }
  }
  None
}