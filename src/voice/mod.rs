use regex::Regex;
use std::{io, time::Duration};

use crate::config;

#[derive(Debug)]
pub struct Voice {
    pub text: String,
    pub bytes: Vec<u8>,
    pub duration: Duration,
}

pub fn create_voices(config: &config::Voice, texts: Vec<String>) -> Result<Vec<Voice>, String> {
    let mut voices = Vec::new();
    for text in texts {
        voices.push(create_voice(config, text)?);
    }
    Ok(voices)
}

fn create_voice(config: &config::Voice, text: String) -> Result<Voice, String> {
    let config::Voice {
        language,
        gender,
        pitch,
        rate,
    } = config;

    let text_filtered = remove_emojis(&text);

    let url = format!("https://texttospeech.responsivevoice.org/v1/text:synthesize?text={text_filtered}&lang={language}&engine=g1&name=&pitch={pitch}&rate={rate}&volume=1&key=kvfbSITh&gender={gender}");

    let attempt = || -> Result<(Vec<u8>, Duration), String> {
        let response = reqwest::blocking::get(&url).map_err(|err| format!("{err:?}"))?;

        let bytes = response.bytes().map_err(|err| format!("{err:?}"))?.to_vec();

        let duration = get_audio_duration(&bytes).map_err(|err| format!("{err:?}"))?;

        Ok((bytes, duration))
    };

    const MAX_ATTEMPTS: usize = 10;

    let mut i = 0;
    loop {
        i += 1;

        match attempt() {
            Ok((bytes, duration)) => {
                return Ok(Voice {
                    bytes,
                    duration,
                    text,
                })
            }

            Err(err) => {
                eprintln!(
                    "[warning] (Attempt {i}/{MAX_ATTEMPTS}): Failed to create voice line - {err:?}"
                );

                if i >= MAX_ATTEMPTS {
                    return Err(err);
                }
            }
        };
    }
}

fn get_audio_duration(bytes: &[u8]) -> Result<Duration, mp3_duration::MP3DurationError> {
    let mut cursor = io::Cursor::new(bytes);
    let duration = mp3_duration::from_read(&mut cursor)?;
    Ok(duration)
}

fn remove_emojis(text: &str) -> String {
    let regex = Regex::new(concat!(
        "[",
        "\u{01F600}-\u{01F64F}", // emoticons
        "\u{01F300}-\u{01F5FF}", // symbols & pictographs
        "\u{01F680}-\u{01F6FF}", // transport & map symbols
        "\u{01F1E0}-\u{01F1FF}", // flags (iOS)
        "\u{002702}-\u{0027B0}",
        "\u{0024C2}-\u{01F251}",
        "]+",
    ))
    .unwrap();

    regex.replace_all(&text, "").to_string()
}
