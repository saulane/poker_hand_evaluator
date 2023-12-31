use crate::cards::{Card, STR_RANKS, STR_SUITS};
use rand::seq::SliceRandom;

pub struct Deck {
    pub cards: Vec<u32>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<u32> = vec![0; 52];
        let mut n = 0;
        let mut rng = rand::thread_rng();

        for r in STR_RANKS.chars() {
            for c in STR_SUITS.chars() {
                let str_card = format!("{}{}", r, c);
                deck[n] = Card::new(&str_card).bit_value;
                n += 1;
            }
        }

        deck.shuffle(&mut rng);
        Deck { cards: deck }
    }

    pub fn remove(&mut self, card: Card) -> Option<Card> {
        let index = self.cards.iter().position(|&x| x == card.bit_value);
        let res = match index {
            Some(i) => Some(Card::from_bit_value(self.cards.remove(i))),
            None => None,
        };
        res
    }

    pub fn draw(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        return Card::from_bit_value(self.cards.pop().unwrap());
    }

    pub fn reset(&mut self) {
        let mut deck: Vec<u32> = vec![0; 52];
        let mut n = 0;

        for r in STR_RANKS.chars() {
            for c in STR_SUITS.chars() {
                let str_card = format!("{}{}", r, c);
                deck[n] = Card::new(&str_card).bit_value;
                n += 1;
            }
        }

        self.cards = deck;
    }
}
