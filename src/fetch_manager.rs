use std::path::{Path, PathBuf};

use crate::fetcher::{fetch_m3u8_file, fetch_save_segments};
use anyhow::Result;
use url::Url;

use reqwest::Client;
use tokio;

#[tokio::main]
pub async fn get_m3u8_file(url: &Url, dir_path: &Path) -> Option<PathBuf> {
  println!("Fetch and save m3u8 file...");
  let client = Client::new();
  let m3u8_filename = String::from("index.m3u8");
  let m3u8_path = dir_path.join(&m3u8_filename);

  match fetch_m3u8_file(&client, url, dir_path, m3u8_filename).await {
    Ok(..) => return Some(m3u8_path),
    Err(_) => return None,
  };
}

fn get_base_url(url: Url) -> Result<Url> {
  let mut splited_url: Vec<&str> = url.as_str().split("/").collect();
  splited_url.pop();
  let base_url = splited_url.join("/");
  let parsed_url = Url::parse(&base_url)?;
  return Ok(parsed_url);
}

fn generate_filenames(segments_len: u32) -> Vec<String> {
  let filenames: Vec<String> = (0..segments_len)
    .map(|x| format!("{}.ts", x))
    .collect::<Vec<String>>();
  return filenames;
}

#[tokio::main]
pub async fn get_segments(raw_url: Url, segments_len: u32, dir_path: &Path) -> Result<()> {
  println!("Fetching segments :");
  let client = Client::new();
  let base_url = get_base_url(raw_url)?;

  let filenames = generate_filenames(segments_len);

  fetch_save_segments(filenames, &client, base_url, dir_path).await;

  return Ok(());
}
