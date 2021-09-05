use bytes::Bytes;
// use rust_embed::RustEmbed;
use std::{
  fs::File,
  io::{BufReader, Result, Write},
  path::{Path, PathBuf},
};

pub fn read_file_to_buffer(path: PathBuf) -> Result<BufReader<File>> {
  let file = File::open(path)?;
  let buffer = BufReader::new(file);

  return Ok(buffer);
}

pub fn write_file_from_buffer(path: PathBuf, buffer: Bytes) -> Result<()> {
  let mut file = File::create(path)?;
  file.write_all(&buffer)?;

  return Ok(());
}

pub fn generate_segments_info(dir_path: &Path, segments_len: u32) -> Result<()> {
  println!("Generate metadata of segments...");
  let path = dir_path.join("segments.txt");
  let mut file = File::create(path)?;
  for index in 0..segments_len {
    write!(file, "file '{}.ts'\n", index)?;
  }

  return Ok(());
}
