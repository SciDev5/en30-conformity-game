use std::io::BufRead;

use crate::{ansi::STYLE_RESET, cfg::Score, character::Character};

pub mod colors;
use colors::*;
use rand::seq::SliceRandom;

pub struct YouSpeaker;
impl Speaker for YouSpeaker {
    const NAME: &'static str = "YOU";
}

pub fn format_name(name: &str) -> String {
    format!("{}{name}{}", STYLE_NAME, STYLE_RESET)
}

pub trait Speaker {
    const NAME: &'static str;

    fn say(&self, text: String) {
        println!(
            "{}: \"{}{text}{}\"",
            format_name(Self::NAME),
            STYLE_SAY,
            STYLE_RESET
        );
    }
    fn act(&self, text: String) {
        println!(
            "{} {}{text}{}",
            format_name(Self::NAME),
            STYLE_ACT,
            STYLE_RESET
        );
    }
    fn prompt_from(&self, prompt: String, responses: Vec<String>) -> usize {
        println!(
            "{} asks, \"{}{prompt}{}\"",
            format_name(Self::NAME),
            STYLE_SAY,
            STYLE_RESET
        );
        for (i, response) in responses.iter().enumerate() {
            println!(
                "{} :: \"{}{}{}\"",
                i + 1,
                STYLE_RESPONSE_OPT,
                response,
                STYLE_RESET
            );
        }

        prompt_raw(responses)
    }
    fn prompt_to(&self, responses: Vec<String>) -> usize {
        println!("You tell {} ...", format_name(Self::NAME));
        for (i, response) in responses.iter().enumerate() {
            println!(
                "{} \"{}{}{}\"",
                i + 1,
                STYLE_RESPONSE_OPT,
                response,
                STYLE_RESET
            )
        }

        prompt_raw(responses)
    }
}

pub fn input_raw() -> String {
    let mut read_in;
    read_in = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut read_in)
        .expect("Failed to read stdin");
    read_in
}

fn prompt_raw(responses: Vec<String>) -> usize {
    let chosen = loop {
        let read_in = input_raw();
        if let Ok(n) = usize::from_str_radix(read_in.trim(), 10) {
            if n <= responses.len() && n != 0 {
                break n - 1;
            }
        }
        println!(
            "{}Enter the number corresponding to the response [1 - {}].{}",
            STYLE_PLAYER_INSTRUCTIONS,
            responses.len(),
            STYLE_RESET,
        );
    };

    YouSpeaker.say(responses[chosen].clone());
    prompt_wait();

    chosen
}

pub fn prompt_wait() {
    let mut read_in = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut read_in)
        .expect("Failed to read stdin");
    if !read_in.trim().is_empty() {
        println!(
            "{} !! input ignored !!  now is not the time to speak.{}",
            STYLE_PLAYER_INSTRUCTIONS, STYLE_RESET
        );
    }
}

#[derive(Clone)]
pub enum Prompt {
    FromPlayer,
    FromCharacter(&'static str),
}
impl Prompt {
    pub fn show<T: Speaker>(&self, character_speaker: &T, responses: Vec<String>) -> usize {
        match self {
            Self::FromPlayer => character_speaker.prompt_to(responses),
            Self::FromCharacter(prompt) => {
                character_speaker.prompt_from(prompt.to_string(), responses)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum ChatSequenceLine {
    PlayerSpeaks(&'static str),
    CharacterDoes(&'static str),
    CharacterSpeaks(&'static str),
}
impl ChatSequenceLine {
    fn show<T: Speaker>(&self, character_speaker: &T) {
        match self {
            ChatSequenceLine::CharacterSpeaks(line) => character_speaker.say(line.to_string()),
            ChatSequenceLine::CharacterDoes(line) => character_speaker.act(line.to_string()),
            ChatSequenceLine::PlayerSpeaks(line) => YouSpeaker.say(line.to_string()),
        };
        prompt_wait();
    }
}

#[derive(Clone)]
pub struct ChatSequence {
    leadup_chat: Vec<ChatSequenceLine>,
    prompt: Prompt,
    responses: Vec<(Score, &'static str)>,
}

impl ChatSequence {
    pub fn new(
        leadup_chat: Vec<ChatSequenceLine>,
        prompt: Prompt,
        responses: Vec<(Score, &'static str)>,
    ) -> Self {
        Self {
            leadup_chat,
            prompt,
            responses,
        }
    }
    pub fn play<T: Character>(&self, character: &T) -> Score {
        for line in &self.leadup_chat {
            line.show(character);
        }
        let mut responses = self.responses.clone();
        responses.shuffle(&mut rand::thread_rng());
        let res_i = self.prompt.show(
            character,
            responses
                .iter()
                .map(|(_, response_text)| response_text.to_string())
                .collect(),
        );

        let (response_score_change, _) = responses[res_i];

        response_score_change
    }
}
