mod cards;
mod lookup;
mod evaluator;
mod deck;
mod tests;
mod game;


use cards::Card;
use evaluator::{eval_5cards, eval_7hand};
use deck::Deck;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(long)]
    c1: String,
    /// Name of the person to greet
    #[arg(long)]
    c2: String,

    #[arg(short, long, default_value_t=1000)]
    n: u128
}

fn main() {
    let args = Cli::parse();

    println!("Your Hand: {}, {}!", args.c1,args.c2);

    let card1 = Card::new(&args.c1);
    let card2 = Card::new(&args.c2);
    let hand = vec![card1, card2];

    let mut wins = 0;
    for _ in 0..args.n{
        let mut deck = Deck::new();
        deck.remove(card1.bit_value);
        deck.remove(card2.bit_value);

        let hand_opp = vec![deck.draw(), deck.draw()];
        // deck.remove(hand_opp[0].bit_value);
        // deck.remove(hand_opp[1].bit_value);


        let board = vec![deck.draw(), deck.draw(), deck.draw(), deck.draw(), deck.draw()];
        // let mut board = vec![Card::new("Th"),Card::new("2s"),Card::new("4c"),Card::new("7d"),Card::new("Qd")];

        
        let r1 = eval_7hand(&board, &hand);
        let r2 = eval_7hand(&board, &hand_opp);
        if  r1 < r2 {
            wins += 1;
        }
    }

    println!("Win rate for {} simulations: {}",args.n, wins as f32 / args.n as f32)

    
}
