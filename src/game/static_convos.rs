use crate::{speaking::{Speaker, YouSpeaker, colors::*, prompt_wait}, character::Character};

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

pub fn play_win_conversation<T: Character>(character: T) {
    YouSpeaker.say(format!("Ok, {}, it seems I have to move on to my next client. It was nice meeting you. I'm anxiously awaiting your next visit!", T::NAME));
    prompt_wait();
    character.say(format!("It's probably best I stay at the Facility for a bit. {}", character.win_quote()));
    prompt_wait();
    character.say(format!("Thank you! I feel like you've really helped me!"));
    prompt_wait();
    character.say(format!("I was worried it was just me who felt that society was not right for me, but now I know the {}DANGERS OF CONFORMITY{}!", STYLE_IMPORTANT, STYLE_SAY));
    prompt_wait();
    character.act(format!("exits the room. It seems that they have begun their journey to a happy ending."));
    prompt_wait();
    BossSpeaker.act(format!("catches you as you exit the room a few minutes after {}.", T::NAME));
    prompt_wait();
    BossSpeaker.say(format!("Nice work! I knew you knew your stuff."));
    prompt_wait();

    println!("GAME OVER, YOU WIN!");
}

pub fn play_lose_conversation<T: Character>(character: T) {
    YouSpeaker.say(format!("Ok, {}, it seems I have to move on to my next client. It was nice meeting you! I'm awaiting your next visit!", T::NAME));
    prompt_wait();
    character.say(format!("Thank you..."));
    prompt_wait();
    character.act(format!("exits the room. It seems that they have not been helped by your efforts."));
    prompt_wait();
    BossSpeaker.act(format!("catches you as you exit the room a few minutes after {}.", T::NAME));
    prompt_wait();
    BossSpeaker.say(format!("That client looked worse than when you started with them, your performance is not living up to my expectations in the slightest."));
    prompt_wait();
    BossSpeaker.say(format!("We're firing you, we need someone better for {}.", T::NAME));
    prompt_wait();

    println!("GAME OVER, YOU LOSE!");

}