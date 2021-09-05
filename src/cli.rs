use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

// const QUALITY: &[&str] = &["chunked", "720p60", "720p", "480p", "360p", "160p"];

#[derive(StructOpt, Debug)]
#[structopt(name = "m3u8dl", about = "Download and Convert m3u8 Very Fast !")]
pub struct CommandLineArgs {
  #[structopt(help = "The URL of the .m3u8 file.")]
  pub url: Url,

  // #[structopt(
  //   possible_values(QUALITY),
  //   help = "VOD quality, some quality may not be available depending to the stream (chunk is the universal and best quality)"
  // )]
  // pub quality: String,
  #[structopt(parse(from_os_str), short, long)]
  pub m3u8_file: Option<PathBuf>,

  #[structopt(parse(from_os_str), short, long)]
  pub ffmpeg_path: Option<PathBuf>,

  #[structopt(help = "File to save the VOD (Any FFMPEG's suppported format will work).")]
  pub output_path: PathBuf,
}
