mod cards;
mod deck;
mod evaluator;
mod game;
mod lookup;
mod range;
mod tests;

use cards::Card;
use clap::Parser;
use deck::Deck;
use evaluator::eval_7hand;
use range::Range;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    hand: String,

    #[arg(short, long, default_value_t = 10000)]
    n: u128,

    #[arg(long, default_value_t = 6)]
    n_players: u128,
}

fn simulate_hand_vs_range(hand: &Vec<Card>, range: &Range) -> f64 {
    let n = 100000;
    let mut wins = 0;
    for _ in 0..n {
        let mut deck = Deck::new();
        deck.remove(hand[0]);
        deck.remove(hand[1]);

        let hands_opp = range.clone().draw(Some(hand.clone()));
        deck.remove(hands_opp[0]);
        deck.remove(hands_opp[1]);

        let board = vec![
            deck.draw(),
            deck.draw(),
            deck.draw(),
            deck.draw(),
            deck.draw(),
        ];
        if eval_7hand(&board, &hand) < eval_7hand(&board, &hands_opp) {
            wins += 1;
        }
    }

    wins as f64 / n as f64
}

fn main() {
    let args = Cli::parse();

    println!("Your Hand: {}", args.hand);

    let card1 = Card::new(&args.hand[0..2]);
    let card2 = Card::new(&args.hand[2..]);
    let hand = vec![card1, card2];

    let hand_opp_range = Range::utg_range();

    let winrate = simulate_hand_vs_range(&hand, &hand_opp_range);

    println!("Win rate for {} simulations: {}", args.n, winrate)
}
