run:
  cargo run \
    && echo "Press ENTER to open video..." \
    && read \
    && nohup xdg-open video.mp4 &> /dev/null

cmd:
  sh cmd && nohup xdg-open video.mp4 &> /dev/null

