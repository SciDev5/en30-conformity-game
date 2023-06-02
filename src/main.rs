use crate::{character::edna::Edna, game::Game};

mod ansi;
mod cfg;
mod character;
mod game;
mod speaking;

fn main() {
    println!("The Interdimensional Literary Figure Rescue Squad\u{2122}");

    let won = Game::new(Edna).play();

    dbg!(won);
}
