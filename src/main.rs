use anyhow::anyhow;
use structopt::StructOpt;
use tempfile::tempdir;

mod cli;
use cli::CommandLineArgs;
mod file_manager;
use file_manager::{generate_segments_info, read_file_to_buffer};
mod parser;
use parser::parse_m3u8_buffer_to_segments_len;
mod fetch_manager;
use fetch_manager::{get_m3u8_file, get_segments};
mod commander;
use commander::{assemble_segments, convert_and_save_vod, get_ffmpeg_path};
mod fetcher;

fn main() -> anyhow::Result<()> {
  let CommandLineArgs {
    url,
    m3u8_file,
    ffmpeg_path,
    output_path,
  } = CommandLineArgs::from_args();
  let dir = tempdir()?;
  let ffmpeg = ffmpeg_path.or_else(get_ffmpeg_path).ok_or(anyhow!(
    "FFMPEG not found ! (try to add ffmpeg's path manually -f )"
  ))?;

  let m3u8_file_path = m3u8_file
    .or(get_m3u8_file(&url, dir.path()))
    .ok_or(anyhow!(
      "Failed to get m3u8 file, try to pass it directly !"
    ))?;

  let m3u8_file_buffer = read_file_to_buffer(m3u8_file_path)?;

  let segments_len = parse_m3u8_buffer_to_segments_len(m3u8_file_buffer)?;
  println!("There are {} segments !", segments_len);

  get_segments(url, segments_len, dir.path())?;
  generate_segments_info(dir.path(), segments_len)?;
  assemble_segments(&ffmpeg, dir.path())?;
  convert_and_save_vod(&ffmpeg, dir.path(), &output_path)?;

  return Ok(());
}
