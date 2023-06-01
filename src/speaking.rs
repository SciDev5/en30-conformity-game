use std::io::BufRead;

use crate::{cfg::Score, character::Character};

pub struct YouSpeaker;
impl Speaker for YouSpeaker {
    const NAME: &'static str = "YOU";
}

pub trait Speaker {
    const NAME: &'static str;

    fn say(&self, text: String) {
        println!("{}: \"{text}\"", Self::NAME);
    }
    fn prompt_from(&self, prompt: String, responses: Vec<String>) -> usize {
        println!("{} asks, \"{prompt}\"", Self::NAME);
        for (i, response) in responses.iter().enumerate() {
            println!("{} :: \"{}\"", i + 1, response)
        }

        prompt_raw(responses)
    }
    fn prompt_to(&self, responses: Vec<String>) -> usize {
        println!("You tell {} ...", Self::NAME);
        for (i, response) in responses.iter().enumerate() {
            println!("{} \"{}\"", i + 1, response)
        }

        prompt_raw(responses)
    }
}

fn prompt_raw(responses: Vec<String>) -> usize {
    let mut read_in;
    let chosen = loop {
        read_in = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut read_in)
            .expect("Failed to read stdin");
        if let Ok(n) = usize::from_str_radix(read_in.trim(), 10) {
            if n <= responses.len() && n != 0 {
                break n - 1;
            }
        }
        println!(
            "Enter the number corresponding to the response [1 - {}].",
            responses.len()
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
        println!(" !! input ignored !!  now is not the time to speak.");
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
    Player(&'static str),
    Character(&'static str),
}
impl ChatSequenceLine {
    fn show<T: Speaker>(&self, character_speaker: &T) {
        match self {
            ChatSequenceLine::Character(line) => character_speaker.say(line.to_string()),
            ChatSequenceLine::Player(line) => YouSpeaker.say(line.to_string()),
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
        let res_i = self.prompt.show(
            character,
            self.responses
                .iter()
                .map(|(_, response_text)| response_text.to_string())
                .collect(),
        );

        let (response_score_change, _) = self.responses[res_i];

        response_score_change
    }
}
