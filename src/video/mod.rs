mod ffmpeg;

use self::ffmpeg::FFMpegCommand;
use crate::{config::Config, voice::Voice};

pub fn concat_voices(config: &Config, dir: &str) {
    let mut ffmpeg = FFMpegCommand::new(config.out.overwrite);

    // Filter: concatenate
    ffmpeg.args(["-f", "concat"]);

    // Input: text file of audio file paths
    ffmpeg.args(["-i", &format!("{dir}/voices.txt")]);

    // Copy data without re-encoding
    ffmpeg.args(["-c", "copy"]);

    // Output path
    ffmpeg.arg(format!("{dir}/audio.mp3"));

    ffmpeg.show_command();
    ffmpeg.run();
}

pub fn render_video(config: &Config, dir: &str, _voices: Vec<Voice>) {
    let mut ffmpeg = FFMpegCommand::new(config.out.overwrite);

    // Background video input
    ffmpeg.args(["-i", &config.assets.background]);

    ffmpeg.args(["-i", &format!("{dir}/voices.txt")]);

    // Trim video to duration of all audio
    ffmpeg.args(["-ss", "00:00:00", "-to", "00:00:10"]);

    // Output file
    ffmpeg.arg(&config.out.name);

    ffmpeg.run();
}
