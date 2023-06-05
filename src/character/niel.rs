use crate::{
    ansi::ANSIModifier::Italic,
    game::ChatPool,
    speaking::{colors::STYLE_SUBTLE, ChatSequence, ChatSequenceLine as Ln, Prompt, Speaker},
};

use super::Character;

pub struct Neil;
impl Speaker for Neil {
    const NAME: &'static str = "NIEL";
}

impl Character for Neil {
    fn origin_citation(&self) -> Option<String> {
        Some(
            format!(
                "Peter Weir, Maurice Jarre, and Patrick Russ. {}DEAD POET'S SOCIETY{}. USA, 1989.",
                Italic, STYLE_SUBTLE
            )
            .trim()
            .to_string(),
        )
    }
    fn backstory(&self) -> &'static str {
        r#"
Neil was recovered with a gun in his hand in the
study of his father.

His father has been restricting his display of
his passions and hobbies, namely Acting. It was
noted that Neil's entire life had just been
uprooted by his father due to his performance
in A Midsummer Night's Dream against his
father's will. His father told him "Tomorrow
I'm withdrawing you from Welton and enrolling
you in Braden Military School. You're going to
Harvard and you're gonna be a doctor."

Upon Triage, it appears he is deeply upset at
his father because he will not listen to his
desires to pursue his passions.

It may be possible to aid him in properly
speaking up against his father to assert his
will to pursue his passions.
        "# // (Schulman)
        .trim()
    }
    fn generate_chat_pool(&self) -> crate::game::ChatPool {
        ChatPool::new(vec![
            (
                20,
                ChatSequence::new(
                    vec![
                        Ln::Character("h"),
                    ],
                    Prompt::FromPlayer,
                    vec![
                        (-50, "answer"),
                        (-20, "answer"),
                        (20, "answer"),
                        (50, "answer"),
                    ],
                ),
            ),
            (
                40,
                ChatSequence::new(
                    vec![],
                    Prompt::FromPlayer,
                    vec![
                        (-50, "answer"),
                        (-20, "answer"),
                        (20, "answer"),
                        (50, "answer"),
                    ],
                ),
            ),
            (
                60,
                ChatSequence::new(
                    vec![],
                    Prompt::FromPlayer,
                    vec![
                        (-50, "answer"),
                        (-20, "answer"),
                        (20, "answer"),
                        (50, "answer"),
                    ],
                ),
            ),
        ])
    }
}
