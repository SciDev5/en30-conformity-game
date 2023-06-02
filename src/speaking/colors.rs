use crate::ansi::{ANSIColor::*, ANSIModifier::*, ANSI};

pub const STYLE_NAME: ANSI<1> = ANSI::cm(LightCyan.fg(), [Bold]);
pub const STYLE_RESPONSE_OPT: ANSI<0> = ANSI::c(LightGreen.fg());
pub const STYLE_SAY: ANSI<0> = ANSI::c(Yellow.fg());
pub const STYLE_ACT: ANSI<1> = ANSI::m([Italic]);

pub const STYLE_IMPORTANT: ANSI<1> = ANSI::cm(LightBlue.fg(), [Bold]);
pub const STYLE_SUBTLE: ANSI<0> = ANSI::c(LightBlack.fg());

pub const STYLE_PLAYER_INSTRUCTIONS: ANSI<1> = ANSI::cm(LightBlack.fg(), [Italic]);
