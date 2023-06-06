use crate::{
    ansi::STYLE_RESET,
    game::ChatPool,
    speaking::{colors::*, format_name, Speaker},
};

pub mod edna;
pub mod niel;

pub trait Character: Speaker {
    /// The work the character is from, in MLA format.
    fn origin_citation(&self) -> Option<String>;
    /// The character's backstory.
    /// Answers "How did they get here"
    fn backstory(&self) -> &'static str;
    /// Print out the character's backstory.
    /// See [`Self::backstory()`].
    fn print_backstory(&self) {
        if let Some(citation) = self.origin_citation() {
            println!("{}{}{}", STYLE_SUBTLE, citation, STYLE_RESET);
        }
        println!("{}'s backstory:", format_name(Self::NAME));
        for line in self.backstory().lines() {
            println!("| {}{}{}", STYLE_IMPORTANT, line, STYLE_RESET);
        }
        println!();
    }
    /// What the character says if you win.
    fn win_quote(&self) -> &'static str;

    /// Generate a list of all dialogue questions for
    /// the given character over the course of the game.
    fn generate_chat_pool(&self) -> ChatPool;
}
