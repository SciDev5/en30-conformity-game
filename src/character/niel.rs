use crate::{
    ansi::ANSIModifier::Italic,
    game::ChatPool,
    speaking::{
        colors::STYLE_SUBTLE,
        ChatSequence,
        ChatSequenceLine::{CharacterDoes, CharacterSpeaks, PlayerSpeaks},
        Prompt, Speaker,
    },
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
    fn win_quote(&self) -> &'static str {
        "I'm gonna need some time to build up confidence to stand up to my dad."
    }
    fn generate_chat_pool(&self) -> crate::game::ChatPool {
        /*
                need to reinforce:
                - A :: art self expression, good
                - B :: he's basically an adult, and he knows what he's doing. doesn't need to listen to his father
                - C :: standing up against his father is an art form
        */
        ChatPool::new(vec![
            (
                20,
                ChatSequence::new(
                    vec![
                        PlayerSpeaks("Tell me more about Mr. Keeting. I gather that he was important to you and your friends."),
                        CharacterSpeaks("He's probably the best teacher at all of Welton, he beleived in trying to teach us to be free thinkers."),
                        PlayerSpeaks("Tell me more, did you talk with him about your father?"),
                        CharacterSpeaks("Yeah. Mr. Keeting once told me something about my father that stuck with me: \"Then you're acting for him, too. You're 
                        playing the part of the dutiful son.\""),
                        PlayerSpeaks("That's an interesting point that Keeting makes!")
                    ],
                    Prompt::FromPlayer,
                    vec![
                        // Builds on Keeting's teachings, helps build Neil's damaged confidence around his father, to give him what he needs to rebel to safety.
                        (50, "It may also be possible to reapply your acting skills in the opposite way. Act confident and assertive, you can convince your father your path is right, or convince him it's time for you to go."),
                        // Help to reinforce Keeting's teachings, but does not build on them, mildly helpful.
                        (20, "He was a good teacher, helping his students recognise they have the ability to be their own person."),
                        // Counterproductive, invalidates Keeting's building on his confidence, and disencourages him from trying to find a way out.
                        (-30, "Unfortunatly, it's not a good point, as you're clearly not acting around your father, you're just there."),
                        // Counterproductive, makes him feel worse about his favorite teacher, while instilling a guilt in him for something that ocurred after his time.
                        (-50, "But that's not the kind of values a teacher should be instilling in their students. It's for the best he was fired."),
                    ],
                ),
            ),
            (
                40,
                ChatSequence::new(
                    vec![
                        PlayerSpeaks("Would you mind reciting me part of your role in the play?"),
                        CharacterSpeaks("Ah, sure why not."),
                        CharacterDoes("stands up taking a large breath in and out."),
                        CharacterSpeaks("\nIf we shadows have offended,\nThink but this, and all is mended,\nThat you have but slumber'd here\nWhile these visions did appear."),
                        PlayerSpeaks("That was quite good!"),
                        CharacterSpeaks("My dad saw that part, I wonder what he was thinking?"),
                    ],
                    Prompt::FromPlayer,
                    vec![
                        // A call to action, citing a need to be free of his dad's control, which is what he needs in order to live.
                        (50, "Art and self-expression go hand in hand, he may have been shocked to see you performing so well, and saw your unapproved acting as a rebellion. You're basically an adult, you can make your own decisions now, keep up your art, do not conform to your father's overly-strict expectations."),
                        // False information, will sen dhim back to his father, but encourage him to fight back.
                        (0, "He definitely saw your passion, all thats left now is to show your father how that's better than being a lawyer."),
                        // Does not answer the question, also too speculative and likely false. The clock is ticking.
                        (-20, "It's possible that for a moment he saw your passion, but decided not to listen."),
                        // This answer is clearly false, and incinuates that his artistic expression may be causing his problems (counterproductive).
                        (-50, "He probably just doesn't like plays, that must be why he hates your performances so much."),
                    ],
                ),
            ),
            (
                60,
                ChatSequence::new(
                    vec![
                        CharacterSpeaks("I feel like society leves me no room for expression!"),
                        PlayerSpeaks("Sometimes it can be hard, especially with a father like yours."),
                        CharacterDoes("Tell me about it."),
                        PlayerSpeaks("Do you have anyone else you could live with?"),
                        CharacterDoes("looks up, confused."),
                    ],
                    Prompt::FromCharacter("What? You think I should move out? Dad would kill me!"),
                    vec![
                        // References his father's borderline abusive behavior, citing his freedom in contrast with the Dangers of Conformity.
                        (50, "Your father's intolerance of you has gone too far, it's necessary that you be your own, separate person. Free yourself of the Dangers of Conformity."),
                        // Acheives goal, but for the wrong reasons (incorrect information).
                        (20, "He's a terrible person that you need to get away from. He clearly hates you."),
                        // Does not address problem with proper depth. The clock is ticking.
                        (-10, "You should think about it, he doesn't sound very nice."),
                        // Rebuilds damage and corners him back into a dangerous conformity.
                        (-50, "You're right, sorry for suggesting that. He is a good influence with your best interests at heart."),
                    ],
                ),
            ),
        ])
    }
}
