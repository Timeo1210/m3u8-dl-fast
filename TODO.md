This file is not intended to be read by regular user !
# TO ADD
Loading bar
Support all m3u8

Input :
  Required :
    - m3u8 URL
  Optional :
    - m3u8 file

if URL and not file :
  - request FILE and store
  - set file path
if URL and file :
  - Parse file
  - Request all ts concurently
  - Write all ts file in good order in segments.txt
  - FFMPEG group file 

# Fetcher

SEE :
https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest

# FFMPEg group file

SEE : 
https://askubuntu.com/questions/716424/how-to-convert-ts-file-into-a-mainstream-format-losslessly

https://superuser.com/questions/692990/use-ffmpeg-copy-codec-to-combine-ts-files-into-a-single-mp4

all segments must be in same folder
segments.txt same folder
command
```bash
ffmpeg -f concat -i ./segments.txt -c copy ./all.ts
```

convert all.ts to any format
```bash
ffmpeg -i ./all.ts -map 0 -c copy output.{format}
```
  

# NEED

CLIer
Parser
Writer
Fetcher
Processer // ffmpeg

# TARGETS

- x86_64-unknown-linux-gnu
- x86_64-pc-windows-gnu
- x86_64-pc-windows-msvc // need to do


# BUILD

linux
`cargo build --release`

win64
`cargo build --target x86_64-pc-windows-gnu --release`

