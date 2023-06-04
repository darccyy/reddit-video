mod drawtext;
mod ffmpeg;

use std::{fs, time::Duration};

use self::drawtext::{drawtext_filter, DrawtextOptions};
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

pub fn apply_video_audio(config: &Config, dir: &str) {
    let mut ffmpeg = FFMpegCommand::new(config.out.overwrite);

    // Background video input
    ffmpeg.args(["-i", &config.assets.background]);
    // Concatenated audio
    ffmpeg.args(["-i", &format!("{dir}/audio.mp3")]);

    // Use audio file for video audio (mutes video and replaces audio)
    ffmpeg.args(["-map", "0:v:0", "-map", "1:a:0"]);

    // Copy data without re-encoding
    ffmpeg.args(["-c", "copy"]);

    // Output path
    ffmpeg.arg(format!("{dir}/video.mp4"));

    ffmpeg.show_command();
    ffmpeg.run();
}

pub fn render_video(config: &Config, dir: &str, voices: &[Voice]) {
    let mut ffmpeg = FFMpegCommand::new(config.out.overwrite);

    // Background video with voice audio
    ffmpeg.args(["-i", &format!("{dir}/video.mp4")]);

    let drawtext_options = DrawtextOptions {
        font: "Serif".to_string(),
        box_: true,
        ..Default::default()
    };

    let mut total_duration = Duration::ZERO;
    let mut filters = Vec::new();
    for voice in voices {
        let Voice { text, duration, .. } = voice;

        let start = total_duration;
        total_duration += *duration;
        let end = total_duration;

        let filter = drawtext_filter(&drawtext_options, text, start, end);

        filters.push(filter);
    }

    if let Some(watermark) = &config.assets.watermark {
        let drawtext_options = DrawtextOptions {
            x: "w*0.8-text_w/2".to_string(),
            y: "h*0.3-text_h/2".to_string(),
            ..Default::default()
        };

        filters.push(drawtext_filter(
            &drawtext_options,
            watermark,
            Duration::ZERO,
            total_duration,
        ));
    }

    let filepath = format!("{dir}/filter.txt");
    fs::write(&filepath, filters.join(",")).expect("Failed to write temporary filter file");
    ffmpeg.args(["-filter_complex_script", &filepath]);

    // Trim video to duration of all audio
    const OUTRO_TIME: Duration = Duration::from_secs(2);
    ffmpeg.args([
        "-ss",
        "00:00:00",
        "-to",
        &timestamp_from_duration(total_duration + OUTRO_TIME),
    ]);

    // Output file
    ffmpeg.arg(&config.out.name);

    ffmpeg.show_command();
    ffmpeg.run();
}

/// format timestamp (hh:mm:ss) from time in seconds
///
/// todo add milliseconds
fn timestamp_from_duration(duration: Duration) -> String {
    let mut seconds = duration.as_secs();

    let mut minutes = seconds / 60;
    seconds = seconds % 60;

    let hours = minutes / 60;
    minutes = minutes % 60;

    format!(
        "{hh}:{mm}:{ss}",
        hh = hours,
        mm = leading_zeros(minutes, 2),
        ss = leading_zeros(seconds, 2)
    )
}

/// Add leading zero to number, if less than desired digit length
fn leading_zeros(number: u64, length: usize) -> String {
    let number = number.to_string();
    if number.len() < length {
        "0".repeat(length - number.len()) + &number
    } else {
        number
    }
}
