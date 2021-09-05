use std::path::Path;

use anyhow::{Error, Result};
use bytes::Bytes;
use url::Url;

use futures::{stream, StreamExt};
use reqwest::Client;

use crate::file_manager::write_file_from_buffer;

const CONCURRENT_REQUESTS: usize = 8;

#[derive(Debug)]
struct Response {
  filename: String,
  data: Bytes,
}

fn save_response(dir_path: &Path, response: Response) {
  let path = dir_path.join(response.filename);
  write_file_from_buffer(path, response.data).expect("Failed to write some file !");
}

pub async fn fetch_m3u8_file(
  client: &Client,
  url: &Url,
  dir_path: &Path,
  filename: String,
) -> Result<()> {
  let resp = client.get(url.to_owned()).send().await?;
  let data = resp.bytes().await?;
  let to_save = Response { filename, data };

  save_response(dir_path, to_save);

  return Ok(());
}

pub async fn fetch_save_segments(
  filenames: Vec<String>,
  client: &Client,
  base_url: Url,
  dir_path: &Path,
) {
  let bodies = stream::iter(filenames)
    .map(|filename| {
      let segment_url = format!("{}/{}", base_url, filename);
      println!("Fetch {}", segment_url);

      async move {
        let resp = client.get(segment_url).send().await?;
        let data = resp.bytes().await?;
        let to_return = Response { filename, data };
        return Ok::<Response, Error>(to_return);
      }
    })
    .buffer_unordered(CONCURRENT_REQUESTS);

  bodies
    .for_each(|buffer| async {
      match buffer {
        Ok(buffer) => save_response(dir_path, buffer),
        Err(_) => panic!("Failed to unwrap segments !"),
      }
    })
    .await;
}
