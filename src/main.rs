use rand::seq::SliceRandom;

use crate::{
    character::{edna::Edna, niel::Neil},
    game::Game, speaking::input_raw,
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

    println!("Play in infinite mode? (y/N)");
    let infinite_mode = match input_raw().to_lowercase().trim() {
        "y" | "yes" | "true" => true,
        _ => false
    };

    if infinite_mode {
        loop {
            for _ in 0 .. 10 {
                println!();
            }
            println!("!!! PLAYING IN INFINITE MODE !!!");
            run_round();
        }
    } else {
        run_round();
    }
}

fn run_round() {
    for _ in 0 .. 10 {
        println!();
    }
    match &mut [CharacterOptions::Edna, CharacterOptions::Neil][..]
        .choose(&mut rand::thread_rng())
        .unwrap()
    {
        CharacterOptions::Edna => Game::new(Edna).play(),
        CharacterOptions::Neil => Game::new(Neil).play(),
    }
}
