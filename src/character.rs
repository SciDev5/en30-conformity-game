use crate::{
    ansi::STYLE_RESET,
    game::ChatPool,
    speaking::{colors::STYLE_IMPORTANT, format_name, Speaker},
};

pub mod edna;

pub trait Character: Speaker {
    /// The character's backstory.
    /// Answers "How did they get here"
    fn backstory(&self) -> &'static str;
    /// Print out the character's backstory.
    /// See [`Self::backstory()`].
    fn print_backstory(&self) {
        println!("{}'s backstory:", format_name(Self::NAME));
        for line in self.backstory().lines() {
            println!("| {}{}{}", STYLE_IMPORTANT, line, STYLE_RESET);
        }
        println!();
    }

    /// Generate a list of all dialogue questions for
    /// the given character over the course of the game.
    fn generate_chat_pool(&self) -> ChatPool;
}
