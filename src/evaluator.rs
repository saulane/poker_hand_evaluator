use crate::lookup::*;
use crate::cards::Card;
use itertools::Itertools;

fn find_fast(mut u: u32) -> u32{
    let a: u32;
    let b: u32;
    let r: u32;

    u = u.wrapping_add(0xe91aaa35);
    u ^= u >> 16;
    u = u.wrapping_add(u << 8);
    u ^= u >> 4;
    b = (u >> 8) & 0x1ff;
    a = (u.wrapping_add(u << 2)) >> 19;
    r = a ^ HASH_ADJUST[b as usize] as u32;
    
    r
}


pub fn eval_5cards(cards: &Vec<Card>) -> u16{
    let mut q: u32 = (cards[0].bit_value | cards[1].bit_value | cards[2].bit_value | cards[3].bit_value | cards[4].bit_value ) >> 16;

    // This checks for Flushes and Straight Flushes
    if (cards[0].bit_value & cards[1].bit_value & cards[2].bit_value & cards[3].bit_value & cards[4].bit_value &  0xf000) != 0 {
        return FLUSHES[q as usize];
    }

    // This checks for Straights and High Card hands
    if UNIQUE5[q as usize] != 0{
        return UNIQUE5[q as usize].clone();
    }
    // This performs a perfect-hash lookup for remaining hands
    q = (cards[0].bit_value & 0xff) * (cards[1].bit_value & 0xff) * (cards[2].bit_value & 0xff) * (cards[3].bit_value & 0xff) * (cards[4].bit_value & 0xff);
    let idx = find_fast(q as u32) as usize;
    return HASH_VALUES[idx];
}

pub fn eval_7hand(board: &Vec<Card>, hand: &Vec<Card>) -> u16{
    let mut cards: Vec<Card> = Vec::with_capacity(board.len() + hand.len());
    cards.extend_from_slice(board);
    cards.extend_from_slice(hand);

    // cards.iter().combinations(5).map(|x| eval_5cards(&x)).min().unwrap()

    let mut best = 65365;
    for comb in cards.iter().combinations(5).clone(){
        let mut temp = Vec::new();
        for c in comb{
            temp.push(c.clone())
        }
        best = best.min(eval_5cards(&temp)); 
    }
    best
}

pub fn hand_rank(val: u16) -> u16 {
    if val > 6185 {return 0};        // 1277 high card
    if val > 3325 {return 1};         // 2860 one pair
    if val > 2467 {return 2};         //  858 two pair
    if val > 1609 {return 3};  //  858 three-kind
    if val > 1599 {return 4};         //   10 straights
    if val > 322  {return 5};            // 1277 flushes
    if val > 166  {return 6};       //  156 full house
    if val > 10   {return 7};   //  156 four-kind
    return 8;                   //   10 straight-flushes
}