use crate::{
    ansi::STYLE_RESET,
    cfg::Score,
    character::Character,
    speaking::{colors::STYLE_IMPORTANT, format_name, prompt_wait, ChatSequence},
};

use rand::seq::SliceRandom;

const START_SCORE: Score = 0;
const WIN_THRESHOLD: Score = 100;
const CHATSEQ_DRAW_RADIUS: Score = 50;

pub struct Game<T: Character> {
    character: T,
    chat_pool: ChatPool,
    score: Score,
}
impl<T: Character> Game<T> {
    pub fn new(character: T) -> Self {
        Self {
            chat_pool: character.generate_chat_pool(),
            character,
            score: START_SCORE,
        }
    }
    pub fn play(mut self) -> bool {
        self.character.print_backstory();
        println!(
            "Help {} realize the {}DANGERS OF CONFORMITY!{}",
            format_name(T::NAME),
            STYLE_IMPORTANT,
            STYLE_RESET
        );
        prompt_wait();

        while let Some(chat_seq) = self.chat_pool.draw(self.score, CHATSEQ_DRAW_RADIUS) {
            self.score += chat_seq.play(&self.character);
        }
        self.score > WIN_THRESHOLD
    }
}

pub struct ChatPool(Vec<(Score, ChatSequence)>);
impl ChatPool {
    fn draw(&mut self, score: Score, score_thresh: Score) -> Option<ChatSequence> {
        let k: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, (req_score, _))| (req_score - score).abs() < score_thresh)
            .map(|(i, (_, seq))| (i, seq.clone()))
            .collect();
        if let Some((i, seq)) = k.choose(&mut rand::thread_rng()).clone() {
            let r = seq.clone();
            self.0.remove(*i);
            return Some(r);
        }
        None
    }
    pub fn new(sequences: Vec<(Score, ChatSequence)>) -> Self {
        Self(sequences)
    }
}
impl Default for ChatPool {
    fn default() -> Self {
        Self::new(vec![])
    }
}
