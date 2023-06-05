use rand::seq::SliceRandom;

use crate::{
    character::{edna::Edna, niel::Neil},
    game::Game,
};

mod ansi;
mod cfg;
mod character;
mod game;
mod speaking;

enum CharacterOptions {
    Edna,
    Neil,
}

fn main() {
    println!("\nWelcome to \nThe Interdimensional Literary Figure Rescue Squad\u{2122} \n\n");

    match &mut [CharacterOptions::Edna, CharacterOptions::Neil][..]
        .choose(&mut rand::thread_rng())
        .unwrap()
    {
        CharacterOptions::Edna => Game::new(Edna).play(),
        CharacterOptions::Neil => Game::new(Neil).play(),
    }
}
