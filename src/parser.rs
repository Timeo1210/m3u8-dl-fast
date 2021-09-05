use anyhow::{anyhow, Result};
use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn parse_m3u8_buffer_to_segments_len(buffer: BufReader<File>) -> Result<u32> {
  println!("Parsing m3u8 file...");
  let lines: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();
  let reverse_line_iter = lines.iter().rev();

  let mut segments_len: u32 = 0;
  for line in reverse_line_iter {
    if line.contains(".ts") {
      let splited_line: Vec<&str> = line.split(".ts").collect();
      segments_len = splited_line[0].parse::<u32>()?;
      break;
    }
  }
  if segments_len == 0 {
    return Err(anyhow!("Failed to obtain number of segments !"));
  }

  return Ok(segments_len);
}
