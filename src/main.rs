mod cards;
mod deck;
mod evaluator;
mod lookup;
mod range;
mod tests;

use cards::Card;
use clap::Parser;
use csv;
use deck::Deck;
use evaluator::eval_hand;
use range::Range;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::sync::mpsc::{channel, Sender};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    hand1: String,

    #[arg(long)]
    hand2: String,
}

fn main() {
    let args = Cli::parse();

    let mut temp_deck = Deck::new();

    let card1 = Card::new(&args.hand1[0..2]);
    let card2 = Card::new(&args.hand1[2..]);
    let hand1 = vec![card1, card2];

    let card21 = Card::new(&args.hand2[0..2]);
    let card22 = Card::new(&args.hand2[2..]);
    let hand2 = vec![card21, card22];

    temp_deck.remove(card1);
    temp_deck.remove(card2);
    temp_deck.remove(card22);
    temp_deck.remove(card21);

    let board = vec![
        temp_deck.draw(),
        temp_deck.draw(),
        temp_deck.draw(),
        temp_deck.draw(),
        temp_deck.draw(),
    ];

    let res = eval_hand(&board, &hand1);
    let res2 = eval_hand(&board, &hand2);
    let win = res < res2;
    println!("{:?}", win);
}
