use super::{Housing, Crawler};
use crate::Result;
use itertools::izip;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::time::Duration;
use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class};
use log::*;

const URL_PATTERN: &str = r#"https://www.immobilienscout24.de"#;

pub struct ImmoScout {}

impl Crawler for ImmoScout {
  fn crawl_url(&self, url: String) -> Result<Vec<Housing>> {
    self.get_results(url)
  }
  fn match_url(&self, url: String) -> bool {
    lazy_static! {
      static ref RE_URL: Regex = Regex::new(URL_PATTERN).unwrap();
    }
    RE_URL.is_match(&url)
  }
}

impl ImmoScout {
  fn get_results(&self, search_url: String) -> Result<Vec<Housing>> {
    debug!("getting results for {}", search_url);
    lazy_static! {
      static ref RE_P_SEARCH: Regex = Regex::new(r#"/Suche/(?P<ID>.+)/P-\d+"#).unwrap();
      static ref RE_NOP_SEARCH: Regex = Regex::new(r#"/Suche/(?P<ID>.+)/"#).unwrap();
    };
    let mut search_url = search_url.clone();
    if search_url.contains("/P-") {
      let captures: String = RE_P_SEARCH.captures(&search_url).unwrap()["ID"].to_string();
      let search_url_str = &format!("/Suche/{}/P-", captures);
      search_url = search_url_str.to_string();
    } else {
      let captures: String = RE_NOP_SEARCH.captures(&search_url).unwrap()["ID"].to_string();
      let search_url_str = &format!("/Suche/{}/P-", captures);
      search_url = search_url_str.to_string();
    }

    let mut page_no = 1;
    debug!("loading page {}", page_no);
    let page = self.get_page(search_url.clone(), page_no)?;
    let no_of_results = Document::from(page.as_str()).find(Attr("data-is24-qa", "resultlist-resultCount")).nth(0).expect("need resultCount").text();
    let no_of_results: u64 = no_of_results.trim().parse()?;
    let mut entries = self.extract_data(page)?;
    while (entries.len() as u64) < no_of_results {
      page_no = page_no + 1;
      debug!("loading page {}", page_no);
      let new_entries = self.get_page(search_url.clone(), page_no)?;
      let mut new_entries = self.extract_data(new_entries)?;
      entries.append(&mut new_entries);
    }
    Ok(entries)
  }
  fn get_page(&self, url: String, page_no: u64) -> Result<String> {
    let url = format!("{}{}{}", URL_PATTERN, url, page_no);
    let client = reqwest::ClientBuilder::new()
      .connect_timeout(Some(Duration::from_secs(5)))
      .max_idle_per_host(1)
      .build()?;
    Ok(client.get(url.as_str()).send()?.text()?)
  }
  fn extract_data(&self, data: String) -> Result<Vec<Housing>> {
    debug!("extracting page data");
    let document = Document::from(data.as_str());

    let title_elements = document.find(Class("result-list-entry__brand-title"));
    let expose_ids = document.find(Class("result-list-entry__brand-title"));
    let expose_ids = expose_ids.map(|e| -> Result<String> { Ok(e.parent()?.attr("href")?.split("/").last()?.replace(".html", "").to_owned()) });
    let expose_urls = document.find(Class("result-list-entry__brand-title"));
    let expose_urls = expose_urls.map(|e| -> Result<String> { Ok(e.parent()?.attr("href")?.split("/").last()?.replace(".html", "").to_owned()) });
    let expose_urls = expose_urls.map(|x| -> Result<String> { Ok(format!("{}/expose/{}", URL_PATTERN, x?)) });
    let attr_container_els = document.find(Attr("data-is24-qa", "attributes"));
    let address_fields = document.find(Class("result-list-entry__address"));
    
    let exposes = izip!(title_elements, expose_ids,
      expose_urls, attr_container_els, address_fields);

    let mut out = Vec::new();
    for expose in exposes {
      let title = expose.0.text().trim().replace("NEU", "");
      let id = expose.1?.parse()?;
      let url = expose.2?;
      let price = expose.3.children().nth(0)?.text();
      let price = price.to_owned();
      let price = price.trim().split(" ").nth(0)?.trim();
      let price = price.replace(".", "").replace(",", ".");
      let price = Decimal::from_str(&price)?;
      let size = expose.3.children().nth(1)?.text();
      let size = size.to_owned();
      let size = size.trim().split(" ").nth(0)?.trim().replace(",", ".");
      let size = Decimal::from_str(&size)?;
      let rooms = expose.3.children().nth(2)?.text();
      let rooms = rooms.split(" ").nth(0)?;
      let rooms = rooms.replace(",", ".");
      let rooms = Decimal::from_str(&rooms)?;
      let address = expose.4.text();
      out.push(Housing{
        title,id,url,price, size, rooms, address,
      })
    }
    Ok(out)
  }
}