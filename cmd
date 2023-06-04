#!/bin/sh

ffmpeg -y -i background.mp4 -map 0:v:0 -filter_complex "[0]drawtext=font=Serif:fontcolor=white:fontsize=32:box=1:boxborderw=15:boxcolor=black@0.8:x=(w-text_w)/2:y=(h-text_h)/2:enable='between(t, 0, 1.296)':expansion=none:text='what'[0]" -to 00:00:03 video.mp4

