use crate::{
    game::ChatPool,
    speaking::{ChatSequence, ChatSequenceLine, Prompt, Speaker},
};

use super::Character;

pub struct Edna;
impl Speaker for Edna {
    const NAME: &'static str = "EDNA";
}
impl Character for Edna {
    fn backstory(&self) -> &'static str {
        r#"
top text.
bottom text.
        "#
        .trim()
    }

    fn generate_chat_pool(&self) -> ChatPool {
        ChatPool::new(vec![
            (
                0,
                ChatSequence::new(
                    vec![
                        ChatSequenceLine::Player("check this!"),
                        ChatSequenceLine::Character("what?"),
                    ],
                    Prompt::FromPlayer,
                    vec![(-10, "bad ending"), (10, "good ending")],
                ),
            ),
            (
                15,
                ChatSequence::new(
                    vec![],
                    Prompt::FromCharacter("lorem ipsum"),
                    vec![(-10, "H."), (10, "... dolar sit amet")],
                ),
            ),
        ])
    }
}
