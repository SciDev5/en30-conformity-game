use crate::{
    ansi::{ANSIModifier::Italic, STYLE_RESET},
    game::ChatPool,
    speaking::{ChatSequence, ChatSequenceLine as Ln, Prompt, Speaker},
};

use super::Character;

pub struct Edna;
impl Speaker for Edna {
    const NAME: &'static str = "EDNA";
}
impl Character for Edna {
    fn origin_citation(&self) -> Option<String> {
        Some(
            format!("\tChopin, Kate. {Italic}The Awakening{STYLE_RESET}. Penguin Books, 2018.")
                .trim()
                .to_string(),
        )
    }
    fn backstory(&self) -> &'static str {
        r#"
Edna was recovered just before drowning off the
shores of grand isle.

She had become distraught after receiving a note
from her lover, "I love you. Good-by --because I
love you.".

On triage, it appears she has become erratic and
depressed and beleives she has no options for self
exploration, and chose death as the only alternative.

It may be possible to grant her a new life in
modern times that suits her.
"#
        .trim()
    }

    fn generate_chat_pool(&self) -> ChatPool {
        ChatPool::new(vec![
            /*

need to reinforce:
- A :: art self expression, good
- B :: robert still values traditional values, and must be left behind (do not comprimise, live for love)
- C :: affirm conformity was bad for her, even if others seemed to like it


            */
            (
                0,
                ChatSequence::new( // A
                    vec![
                        Ln::Player("What was something you do for fun? First thing to come to mind."),
                        Ln::Character("Well, I like painting and listening to Mademoiselle Reisz's music."),
                        Ln::Player("In that case, I would like to ask if you would want to paint your ideal life."),
                        Ln::CharacterDoes("paints a scene depicting herself and a man sitting in two separate boats, holding hands over the water."),
                        // symbol ocean -> freedom in romantic expression
                        // symbol separate boats -> separation of identity
                        // holding hands -> romance
                    ],
                    Prompt::FromCharacter("What do you think?"),
                    vec![
                        (-30, "One boat on the shore would have been better, it could symbolize how marriage is good for you."),
                        (-10, "The water looks nice."),
                        (10, "Good use of holding hands to convey real connection."),
                        (30, "That's beautiful! The two boats must symbolize a desire to be your own person in a relationship, right?"),
                    ],
                ),
            ),
            (
                20,
                ChatSequence::new(
                    vec![
                    ],
                    Prompt::FromPlayer,
                    vec![
                    (0, "")
                    ],
                )
            ),
            (
                40,
                ChatSequence::new(
                    vec![
                    ],
                    Prompt::FromPlayer,
                    vec![
                    (0, "")
                    ],
                )
            ),
            (
                60,
                ChatSequence::new(
                    vec![
                    ],
                    Prompt::FromPlayer,
                    vec![
                    (0, "")
                    ],
                )
            ),
            (
                80,
                ChatSequence::new(
                    vec![
                    ],
                    Prompt::FromPlayer,
                    vec![
                    (0, "")
                    ],
                )
            ),
            (
                100,
                ChatSequence::new(
                    vec![
                    ],
                    Prompt::FromPlayer,
                    vec![
                    (0, "")
                    ],
                )
            )
        ])
    }
}
