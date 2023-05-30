use regex::Regex;
use std::{io, time::Duration};

use crate::config;

#[derive(Debug)]
pub struct Voice {
    pub bytes: Vec<u8>,
    pub duration: Duration,
}

pub fn create_voices(config: &config::Voice, texts: &[String]) -> Result<Vec<Voice>, String> {
    let mut voices = Vec::new();
    for text in texts {
        voices.push(create_voice(config, text)?);
    }
    Ok(voices)
}

fn create_voice(config: &config::Voice, text: &str) -> Result<Voice, String> {
    let config::Voice {
        language,
        gender,
        pitch,
        rate,
    } = config;

    let text = remove_emojis(text);

    let url = format!("https://texttospeech.responsivevoice.org/v1/text:synthesize?text={text}&lang={language}&engine=g1&name=&pitch={pitch}&rate={rate}&volume=1&key=kvfbSITh&gender={gender}");

    let attempt = || -> Result<Voice, String> {
        let response = reqwest::blocking::get(&url).map_err(|err| format!("{err:?}"))?;

        let bytes = response.bytes().map_err(|err| format!("{err:?}"))?.to_vec();

        let duration = get_audio_duration(&bytes).map_err(|err| format!("{err:?}"))?;

        Ok(Voice { bytes, duration })
    };

    const MAX_ATTEMPTS: usize = 10;

    let mut i = 0;
    loop {
        i += 1;

        match attempt() {
            Ok(value) => return Ok(value),

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
    let emoji_regex = Regex::new(r#"\p{Emoji}"#).unwrap();
    emoji_regex.replace_all(text, "").to_string()
}
