use crate::{
    ansi::ANSIModifier::Italic,
    game::ChatPool,
    speaking::{ChatSequence, ChatSequenceLine as Ln, Prompt, Speaker, colors::STYLE_SUBTLE},
};

use super::Character;

pub struct Edna;
impl Speaker for Edna {
    const NAME: &'static str = "EDNA";
}
impl Character for Edna {
    fn origin_citation(&self) -> Option<String> {
        Some(
            format!("\tChopin, Kate. {Italic}The Awakening{STYLE_SUBTLE}. Penguin Books, 2018.")
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
                ChatSequence::new( // Art can be used to express exporation
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
                        // Supportive symbol recognition.
                        (30, "That's beautiful! The two boats must symbolize a desire to be your own person in a relationship, right?"),
                        // Ditto, but less well executed.
                        (10, "Good use of holding hands to convey real connection."),
                        // Non-answer. The clock is ticking.
                        (-10, "The water looks very nice, and the shading is excellent."),
                        // Actively counterproductive, as we are trying to make her comfortable in nonconformity.
                        (-30, "One boat on the shore would have been better, it could symbolize how marriage is good for you."),
                    ],
                ),
            ),
            (
                20,
                ChatSequence::new( // Robert gave into conformity = bad
                    vec![
                        Ln::Player("Do you still think about Robert?"),
                        Ln::CharacterDoes("looks pensive, showing a mix of longing and betrayal."),
                        Ln::Character("I think about him a lot, but it seems he doesn't want to go against the norms holding us down."),
                    ],
                    Prompt::FromPlayer,
                    vec![
                        // Good. Simultaniously recognises and validates Edna's romantic desires, while encouraging her to be nonconformist.
                        (30, "You can do better than him. Knowing your passion for romance, you can find someone new that doesn't let society make you \"just his wife\"."),
                        // Ditto, bit less well executed.
                        (20, "If he can't see the benefit of defying gender norms, he's not worth your time."),
                        // Robert is an escapist crush of Edna that won't solve her problems, encouraging her to go back is counterproductive.
                        (-20, "He was great, get him back! Even though he gave into conformity."),
                        // Need I explain how terrible this response is.
                        (-30, "Screw him, he sucks. Go back to Léonce."),
                    ],
                ),
            ),
            (
                40,
                ChatSequence::new( // Motherhood vs sexual freedom & personality beyond marriage
                    vec![
                        Ln::Character("They all are so blind to true love, especially Robert."),
                        Ln::Player("Yeah, it does suck when society seems to make unwritten rules that just dont work with you."),
                    ],
                    Prompt::FromCharacter("Why does society seem so focused on making it so I become a mother-woman?"),
                    vec![
                        // It answers the quesiton in a way that reinforces her need to have personal freedom in romance.
                        (30, "It's unfortunately part of the society you left. That is why it's so dangerous to conform, you would lose your identity too much for your liking if you followed blindly."),
                        // Factual response, cold and heartless. Without a rhetorical punchline and in this situation this comes off as insensitive.
                        (-20, "Social norms such as the nuclear family are useful for ensuring the continuation of the human race."),
                        // Ditto, but also encourages her to force herself to conform, despite that being contrary to the goal and bad for her mental health.
                        (-40, "Because it's a natural part of life that you cannot avoid. It would be best if you gave into conformity and devoted yourself to womanhood."),
                    ],
                ),
            ),
            (
                60,
                ChatSequence::new( // Creation of art is closely tied to unconformity
                    vec![
                        Ln::Character("Do you know about my art?"),
                        Ln::CharacterDoes("is sketching a bird flying free in the sky on a peice of paper you gave her earlier."),
                        Ln::Player("I would love to hear about it."),
                        Ln::Character("I've been trying more and more to become an artist ever since my last summer trip to Grand Idle."),
                        Ln::Player("Have you consiered the role of art in shaping your feelings?"),
                    ],
                    // It's a bit on-the-nose, but oh well.
                    Prompt::FromCharacter("No, not really... How is my art connected to my awakening?"),
                    vec![
                        // Illustrates the thematic link between art and conformity illustrated in the book.
                        (20, "Well, it's likely that learning to express yourself through art is linked to learning to express your personal desires, even under the pressure of society."),
                        // Ditto, but makes a connection to the free-bird symbol.
                        // Also, https://youtu.be/zvTWi0eAybk
                        (30, "I think the bird you've drawn illustrates this well. It let you express and reinforce your desires for freedom from motherly conformity."),
                        // Non-answer. The clock is ticking.
                        (-10, "I dont think they're linked."),
                        // Counterproductive, both false and paints art and nonconformity as something which is bad and responsible for her issues.
                        (-30, "Your awakening was probably a side effect of learning art. If you hadn't become an artist you wouldn't be in this situation."),
                    ],
                ),
            ),
            (
                80,
                ChatSequence::new( // Edna does not need to be a mother-woman, conformity bad
                    vec![
                        Ln::CharacterDoes(" sits, looking out the window, doodling a sketch."),
                        Ln::Character("I've been thinking about my place in society..."),
                        Ln::Player("Well, then think back to your old life, what else was good and what else was bad?"),
                        Ln::Character("I was thinking about Adèle, she fits so well into society's definition of a mother-woman."),
                    ],
                    Prompt::FromCharacter("Adèle, everyone else, why do they all seem so complicit in effacing their personal identity to be mothers?"),
                    vec![
                    // Shows her that it's important not to fit into society's box, no matter what social pressures show.
                    (30, "Some enjoy devoting everything to motherhood, but preferred lifestlyes vary, and it's imperative you live yours to the fullest."),
                    // Ditto, but does not address the differences between people that make diversity and nonconformity matter.
                    (20, "Does it matter? You're happier when you keep your freedom. You're no longer confined to their world anymore, live a little!"),
                    // Forces Edna into a box, conter to what the freeing we're trying to accomplish. If you're choosing this option you have got to be stupid.
                    (-40, "Because Adèle's conformist, traditionalist behavior keeps families healthy, something I know you value."),
                    ],
                ),
            ),
        ])
    }
}
