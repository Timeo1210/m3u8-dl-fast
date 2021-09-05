use anyhow::Result;
use std::{
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

pub fn get_ffmpeg_path() -> Option<PathBuf> {
  println!("Finding your ffmpeg path...");
  match Command::new("ffmpeg")
    .stdout(Stdio::null()) // don't verbose to host stdio
    .stderr(Stdio::null()) // don't verbose to host stdio
    .stdin(Stdio::null()) // may be safer ??
    .env_clear() // safer
    .status()
  {
    Ok(..) => return Some(PathBuf::from("ffmpeg")),
    Err(..) => return None,
  }
}

pub fn assemble_segments(ffmpeg: &PathBuf, dir_path: &Path) -> Result<()> {
  println!("Assemble segments using ffmpeg...");
  let segments_info_path = dir_path.join("segments.txt");
  let output_path = dir_path.join("all.ts");

  let segments_info = segments_info_path
    .to_str()
    .expect("Failed to assemble segments !");
  let output = output_path.to_str().expect("Failed to assemble segments !");

  let args = ["-f", "concat", "-i", segments_info, "-c", "copy", output];
  let mut process = Command::new(ffmpeg)
    .args(args)
    .stdout(Stdio::null()) // don't verbose to host stdio
    .stderr(Stdio::null()) // don't verbose to host stdio
    .stdin(Stdio::null()) // may be safer ??
    .env_clear()
    .spawn()
    .expect("Failed to assemble segments !");

  process.wait().expect("Failed to assemble segments !");

  return Ok(());
}

pub fn convert_and_save_vod(
  ffmpeg: &PathBuf,
  dir_path: &Path,
  output_path: &PathBuf,
) -> Result<()> {
  println!("Convert and save VOD using ffmpeg...");
  let vod_path = dir_path.join("all.ts");

  let vod = vod_path.to_str().expect("Failed to convert VOD !");
  let output = output_path
    .to_str()
    .expect("Failed to parse output_path, try other format/path !");

  let args = ["-y", "-i", vod, "-map", "0", "-c", "copy", output];

  let mut process = Command::new(ffmpeg)
    .args(args)
    .stdout(Stdio::null()) // don't verbose to host stdio
    .stderr(Stdio::null()) // don't verbose to host stdio
    .stdin(Stdio::null()) // may be safer ??
    .env_clear()
    .spawn()
    .expect("Failed to convert and save VOD !");

  process
    .wait()
    .expect("Failed to convert and save segments !");

  return Ok(());
}
