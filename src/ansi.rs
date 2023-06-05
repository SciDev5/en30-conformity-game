use core::fmt;
use std::fmt::{Debug, Display, Formatter};

/// Escape symbol for terminal style and color: https://en.wikipedia.org/wiki/ANSI_escape_code
const ANSI_ESCAPE: &str = "\x1B";
pub const STYLE_RESET: &str = "\x1B[0m";

pub struct ANSI<const L: usize> {
    mods: [ANSIModifier; L],
    color: Option<ANSIColorDrawer>,
}
impl ANSI<0> {
    pub const fn c(color: ANSIColorDrawer) -> Self {
        Self {
            mods: [],
            color: Some(color),
        }
    }
}
impl<const L: usize> ANSI<L> {
    pub const fn cm(color: ANSIColorDrawer, mods: [ANSIModifier; L]) -> Self {
        Self {
            mods,
            color: Some(color),
        }
    }
    pub const fn m(mods: [ANSIModifier; L]) -> Self {
        Self { mods, color: None }
    }
}

impl<const L: usize> Display for ANSI<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(color) = &self.color {
            write!(f, "{color}")?
        }
        for modifier in &self.mods {
            write!(f, "{modifier}")?;
        }
        Ok(())
    }
}

/// Represents all different kinds of text styles that the console can render.
#[derive(Debug)]
#[allow(unused)]
pub enum ANSIModifier {
    Bold,
    Italic,
    Underline,
}

impl Display for ANSIModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let id = match self {
            Self::Bold => "1",
            Self::Italic => "3",
            Self::Underline => "4",
        };
        write!(f, "{ANSI_ESCAPE}[{id}m")
    }
}

/// Represents all different kinds of colors that the console can render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum ANSIColor {
    RGB(u8, u8, u8),
    Default,
    Black,
    LightBlack, // I shit you not that is what it's called in the standard
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    White,
    LightWhite,
}

#[allow(unused)]
pub enum ANSIColorDrawer {
    Foreground(ANSIColor),
    Background(ANSIColor),
}
impl Display for ANSIColorDrawer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Foreground(color) => write!(f, "{}", color.stringify(FG)),
            Self::Background(color) => write!(f, "{}", color.stringify(BG)),
        }
    }
}

enum ColorDrawMode {
    FG,
    BG,
}
use ColorDrawMode::*;

impl ANSIColor {
    pub const fn fg(self) -> ANSIColorDrawer {
        ANSIColorDrawer::Foreground(self)
    }
    #[allow(unused)]
    pub const fn bg(self) -> ANSIColorDrawer {
        ANSIColorDrawer::Background(self)
    }
    /// Gets the full ANSI code for this color.
    fn stringify(&self, which: ColorDrawMode) -> String {
        let id = match (which, self) {
            (which, Self::RGB(r, g, b)) => {
                // FG: ESC[38;2;⟨r⟩;⟨g⟩;⟨b⟩m
                // BG: ESC[48;2;⟨r⟩;⟨g⟩;⟨b⟩m
                format!(
                    "{};2;{};{};{}",
                    match which {
                        FG => "38",
                        BG => "48",
                    },
                    r.to_string(),
                    g.to_string(),
                    b.to_string()
                )
            }

            (FG, Self::Default) => "39".to_string(),
            (BG, Self::Default) => "49".to_string(),

            (FG, Self::Black) => "30".to_string(),
            (BG, Self::Black) => "40".to_string(),
            (FG, Self::LightBlack) => "90".to_string(),
            (BG, Self::LightBlack) => "100".to_string(),

            (FG, Self::Red) => "31".to_string(),
            (BG, Self::Red) => "41".to_string(),
            (FG, Self::LightRed) => "91".to_string(),
            (BG, Self::LightRed) => "101".to_string(),

            (FG, Self::Green) => "32".to_string(),
            (BG, Self::Green) => "42".to_string(),
            (FG, Self::LightGreen) => "92".to_string(),
            (BG, Self::LightGreen) => "102".to_string(),

            (FG, Self::Yellow) => "33".to_string(),
            (BG, Self::Yellow) => "43".to_string(),
            (FG, Self::LightYellow) => "93".to_string(),
            (BG, Self::LightYellow) => "103".to_string(),

            (FG, Self::Blue) => "34".to_string(),
            (BG, Self::Blue) => "44".to_string(),
            (FG, Self::LightBlue) => "94".to_string(),
            (BG, Self::LightBlue) => "104".to_string(),

            (FG, Self::Magenta) => "35".to_string(),
            (BG, Self::Magenta) => "45".to_string(),
            (FG, Self::LightMagenta) => "95".to_string(),
            (BG, Self::LightMagenta) => "105".to_string(),

            (FG, Self::Cyan) => "36".to_string(),
            (BG, Self::Cyan) => "46".to_string(),
            (FG, Self::LightCyan) => "96".to_string(),
            (BG, Self::LightCyan) => "106".to_string(),

            (FG, Self::White) => "37".to_string(),
            (BG, Self::White) => "47".to_string(),
            (FG, Self::LightWhite) => "97".to_string(),
            (BG, Self::LightWhite) => "107".to_string(),
        };
        format!("{ANSI_ESCAPE}[{id}m")
    }
}
