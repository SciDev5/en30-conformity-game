use crate::speaking::{Speaker, YouSpeaker, colors::*, prompt_wait};

struct CoworkerSpeaker;
impl Speaker for CoworkerSpeaker {
    const NAME: &'static str = "CO-WORKER";
}

struct BossSpeaker;
impl Speaker for BossSpeaker {
    const NAME: &'static str = "BOSS";
}

/// Played in the beginning of the game
/// to orient the character and help them
/// understand what their goal is.
pub fn play_onboarding_conversation() {
    CoworkerSpeaker.say(format!("Hello!"));
    prompt_wait();
    YouSpeaker.say(format!("Hello!"));
    prompt_wait();
    CoworkerSpeaker.say(format!("You're the new hire, eh? Nice to meet you!"));
    prompt_wait();
    CoworkerSpeaker.act(format!("reaches out their hand and gives you a firm handshake."));
    prompt_wait();
    YouSpeaker.say(format!("So.. how does this work? I know we're a therapy group, is there anything else I should know."));
    prompt_wait();
    CoworkerSpeaker.say(format!("Oh yeah, we're the world's first and only {}INTERDIMENSIONAL-TIME-TRAVEL THERAPY GROUP{}!", STYLE_IMPORTANT, STYLE_SAY));
    prompt_wait();
    YouSpeaker.say(format!("I'm sorry what?"));
    prompt_wait();
    CoworkerSpeaker.say(format!("I'm not kidding! We rescue important characters from other universes who are in desprate need!"));
    prompt_wait();
    BossSpeaker.say(format!("Lunch break's over! We've got new clients today from {}The Awakening{} and {}Dead Poet Society{}, so I'm gonna need new asignees on that!", STYLE_IMPORTANT, STYLE_SAY, STYLE_IMPORTANT, STYLE_SAY));
    prompt_wait();
    CoworkerSpeaker.say(format!("Good luck on your first day!"));
    prompt_wait();
    YouSpeaker.say(format!("Thanks!"));
    prompt_wait();
    YouSpeaker.act(format!("had heard of the opening of this after the invention of time travel. It's fun to know you got a job that utilizes it."));
    prompt_wait();
}