use crate::{character::edna::Edna, game::Game};

mod ansi;
mod cfg;
mod character;
mod game;
mod speaking;

fn main() {
    let won = Game::new(Edna).play();

    dbg!(won);
}
