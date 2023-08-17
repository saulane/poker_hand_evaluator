use crate::lookup::PRIMES;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Assuming STR_RANKS and INT_RANKS are defined earlier in Python
// let's assume they're arrays/vectors in Rust for this translation
// let STR_RANKS: Vec<&str> = vec!["a", "b", ...];
// let INT_RANKS: Vec<i32> = vec![1, 2, ...];
pub const STR_RANKS: &str = "23456789TJQKA";
pub const STR_SUITS: &str = "shdc";
pub const INT_RANKS: [u8; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

const INT_SUIT_TO_CHAR_SUIT: &str = "xshxdxxxc";

lazy_static! {
    static ref CHAR_SUIT_TO_INT_SUIT: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('s', 1);
        m.insert('h', 2);
        m.insert('d', 4);
        m.insert('c', 8);
        m
    };
    static ref CHAR_RANK_TO_INT_RANK: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('2', 0);
        m.insert('3', 1);
        m.insert('4', 2);
        m.insert('5', 3);
        m.insert('6', 4);
        m.insert('7', 5);
        m.insert('8', 6);
        m.insert('9', 7);
        m.insert('T', 8);
        m.insert('J', 9);
        m.insert('Q', 10);
        m.insert('K', 11);
        m.insert('A', 12);
        m
    };
}

// +--------+--------+--------+--------+
// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
// +--------+--------+--------+--------+

// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
// cdhs = suit of card (bit turned on based on suit of card)
// b = bit turned on depending on rank of card

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub bit_value: u32,
    pub rank_char: char,
    pub suit_char: char,
}

impl Card {
    pub fn new(value: &str) -> Card {
        let rank_char = value.chars().nth(0).unwrap();
        let suit_char = value.chars().nth(1).unwrap();

        let rank_int: u32 = CHAR_RANK_TO_INT_RANK[&rank_char] as u32;
        let suit_int = CHAR_SUIT_TO_INT_SUIT[&suit_char] as u32;
        let rank_prime = PRIMES[rank_int as usize] as u32;

        let bitrank = 1 << rank_int << 16;
        let suit = suit_int << 12;
        let rank = rank_int << 8;

        let bit_value: u32 = bitrank as u32 | suit as u32 | rank as u32 | rank_prime as u32;
        Card {
            bit_value,
            rank_char,
            suit_char,
        }
    }

    pub fn from_bit_value(bit_value: u32) -> Card {
        let suit = Card::get_suit_int(bit_value) as usize;
        let rank = Card::get_rank_int(bit_value) as usize;
        Card {
            bit_value,
            rank_char: STR_RANKS.chars().nth(rank).unwrap(),
            suit_char: INT_SUIT_TO_CHAR_SUIT.chars().nth(suit).unwrap(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.rank_char, self.suit_char)
    }

    fn get_suit_int(value: u32) -> u32 {
        (value >> 12) & 0xF
    }

    fn get_rank_int(value: u32) -> u32 {
        (value >> 8) & 0xF
    }
}
