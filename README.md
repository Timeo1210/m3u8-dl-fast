# m3u8-dl-fast

Fastest CLI program to download m3u8 and convert to any format !  

### Why Fastest ?
That because this program fetch segments concurrently. **Up to 2x faster !**

### Formats
You can convert m3u8 to any ffmpeg supported format

## Dependencies

FFMPEG for video conversion, see https://www.ffmpeg.org/download.html

## Install

Download your corresponding OS program (macos not supported) in the release panel.

## Usage

**Support only Twitch .m3u8 file**

`m3u8dlfast [OPTIONS] <url> <output-path>`

FLAGS:  
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:  
    -f, --ffmpeg-path <ffmpeg-path>    
    -m, --m3u8-file <m3u8-file>        

ARGS:  
    <url>            The URL of the .m3u8 file.
    <output-path>    File to save the VOD (Any FFMPEG's suppported format will work).

