use crate::{
    ansi::STYLE_RESET,
    cfg::Score,
    character::Character,
    game::intro::play_onboarding_conversation,
    speaking::{
        colors::STYLE_IMPORTANT, format_name, prompt_wait, ChatSequence, Speaker, YouSpeaker,
    },
};

use rand::seq::SliceRandom;

mod intro;

const START_SCORE: Score = 0;
const WIN_THRESHOLD: Score = 80;
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
    fn show_post_play_hint(&self, delta_score: Score) {
        if delta_score > 15 {
            // Good answer
            self.character.act(format!("looked more upbeat after hearing that."));
        } else if delta_score > 0 {
            // Neutral answer
            self.character.act(format!("seems not to care."));
        } else if delta_score > -15 {
            // Neutral-negative answer
            self.character.act(format!("appears to have been irritated upon hearing that."));
        } else {
            // Bad answer
            self.character.act(format!("is clearly very upset by your answer."));
        }
    }
    fn character_status_text(&self) -> &str {
        if self.score > 70 {
            "on the road to recovery"
        } else if self.score > 50 {
            "doing better"
        } else if self.score > 20 {
            "stable"
        } else if self.score >= 0 {
            "in rough shape"
        } else {
            "looking quite grim"
        }
    }
    pub fn play(mut self) -> bool {
        play_onboarding_conversation();
        self.character.print_backstory();
        prompt_wait();

        YouSpeaker.say(format!("Hello, {}. I'm here to help. I know you've been through a lot so, I'm here for you!", T::NAME));
        prompt_wait();
        YouSpeaker.act(format!("begin to chat with {}, getting to know them and learn best how help them.", T::NAME));
        prompt_wait();
        
        println!(
            "Help {} realize the {}DANGERS OF CONFORMITY!{}",
            format_name(T::NAME),
            STYLE_IMPORTANT,
            STYLE_RESET
        );
        prompt_wait();

        while let Some(chat_seq) = self.chat_pool.draw(self.score, CHATSEQ_DRAW_RADIUS) {
            YouSpeaker.act(format!(
                "chat with {} for about {} minutes. It seems that they are {}. ",
                T::NAME,
                (rand::random::<f32>() * 10.0 + 5.0).floor() as u8,
                self.character_status_text()
            ));
            let delta_score = chat_seq.play(&self.character);
            self.show_post_play_hint(delta_score);
            self.score += delta_score;
            prompt_wait();
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
