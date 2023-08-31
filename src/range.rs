use crate::cards::{Card, STR_RANKS, STR_SUITS};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Range {
    pub hands: Vec<Vec<Card>>,
}

impl Range {
    pub fn new(cards_range: Vec<&str>) -> Range {
        let mut hands: Vec<Vec<Card>> = Vec::new();
        for r in cards_range {
            if r.chars().nth(2).unwrap().to_string() == "+" {
                let pair_rank = r.chars().nth(0).unwrap();
                let start_pos = STR_RANKS.chars().position(|x| x == pair_rank).unwrap();

                for _ in STR_RANKS[start_pos..].chars() {
                    for (i, s1) in STR_SUITS.chars().enumerate() {
                        for (_, s2) in STR_SUITS[i + 1..].chars().enumerate() {
                            let card_1 = Card::new(&format!("{}{}", pair_rank, s1));
                            let card_2 = Card::new(&format!("{}{}", pair_rank, s2));

                            hands.push(vec![card_1, card_2]);
                        }
                    }
                }
            }

            let c1_rank = r.chars().nth(0).unwrap();
            let c2_rank = r.chars().nth(1).unwrap();

            let suited = r.chars().nth(2).unwrap().to_string() == "s";
            let over: bool = match r.chars().nth(2) {
                Some(c) => c.to_string() == "+",
                _ => false,
            };

            if suited {
                if over {
                    for s in STR_SUITS.chars() {
                        let card_1 = Card::new(&format!("{}{}", c1_rank, s));
                        let idx_low_cards = STR_RANKS.chars().position(|x| x == c2_rank).unwrap();
                        for c in STR_RANKS[idx_low_cards..].chars() {
                            let card_2 = Card::new(&format!("{}{}", c, s));
                            hands.push(vec![card_1, card_2]);
                        }
                    }
                } else {
                    for s in STR_SUITS.chars() {
                        let card_1 = Card::new(&format!("{}{}", c1_rank, s));
                        let card_2 = Card::new(&format!("{}{}", c2_rank, s));

                        hands.push(vec![card_1, card_2]);
                    }
                }
            } else {
                if over {
                    let idx_low_cards = STR_RANKS.chars().position(|x| x == c2_rank).unwrap();
                    for (i, s) in STR_SUITS.chars().enumerate() {
                        for (_, s2) in STR_SUITS[i + 1..].chars().enumerate() {
                            let card_1 = Card::new(&format!("{}{}", c1_rank, s));
                            for c in STR_RANKS[idx_low_cards..].chars() {
                                let card_2 = Card::new(&format!("{}{}", c, s2));
                                hands.push(vec![card_1, card_2]);
                            }
                        }
                    }
                } else {
                    for (i, s) in STR_SUITS.chars().enumerate() {
                        for (_, s2) in STR_SUITS[i + 1..].chars().enumerate() {
                            let card_1 = Card::new(&format!("{}{}", c1_rank, s));
                            let card_2 = Card::new(&format!("{}{}", c2_rank, s2));

                            hands.push(vec![card_1, card_2]);
                        }
                    }
                }
            }
        }
        Range { hands }
    }

    pub fn draw(&mut self, filter: Option<Vec<Card>>) -> Vec<Card> {
        let mut rng = rand::thread_rng();

        match filter {
            Some(f) => {
                let mut hands = self.hands.clone();
                hands.retain(|x| !f.contains(&x[0]) && !f.contains(&x[1]));
                hands.choose(&mut rng).unwrap().clone()
            }
            None => self.hands.choose(&mut rng).unwrap().clone(),
        }
    }

    pub fn utg_range() -> Range {
        Range::new(UTG_RANGE.to_vec())
    }

    pub fn utg1_range() -> Range {
        Range::new(UTG1_RANGE.to_vec())
    }

    pub fn co_range() -> Range {
        Range::new(CO_RANGE.to_vec())
    }

    pub fn btn_range() -> Range {
        Range::new(BTN_RANGE.to_vec())
    }
}

const UTG_RANGE: [&str; 13] = [
    "22+", "JTs", "QTs", "KTs", "ATs", "QJs", "KJs", "AJs", "KQs", "AQs", "AJo", "AQo", "AKo",
];

const UTG1_RANGE: [&str; 15] = [
    "22+", "JTs", "QTs", "KTs", "ATs", "QJs", "KJs", "AJs", "KQs", "AQs", "AJo", "AQo", "AKo",
    "KJo", "QJo",
];

const CO_RANGE: [&str; 31] = [
    "22+", "JTs", "QTs", "KTs", "ATs", "QJs", "KJs", "AJs", "KQs", "AQs", "AJo", "AQo", "AKo",
    "KJo", "QJo", "T9s", "J9s", "A9s", "K9s", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s",
    "A2s", "65s", "76s", "87s", "98s",
];

const BTN_RANGE: [&str; 64] = [
    "22+", "JTs", "QTs", "KTs", "ATs", "QJs", "KJs", "AJs", "KQs", "AQs", "AJo", "AQo", "AKo",
    "KJo", "QJo", "T9s", "J9s", "A9s", "K9s", "A9s", "A8s", "A7s", "A6s", "A5s", "A4s", "A3s",
    "65s", "76s", "87s", "98s", "A2s", "A2o", "A3o", "A4o", "A5o", "A6o", "A7o", "A8o", "A9o",
    "K8o", "Q8o", "J8o", "T8o", "98o", "87o", "K9o", "Q9o", "J9o", "T9o", "K8s", "Q8s", "J8s",
    "T8s", "98s", "87s", "K7s", "Q7s", "J7s", "T7s", "K6s", "K5s", "K4s", "K3s", "K2s",
];
