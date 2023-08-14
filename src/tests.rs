mod tests{
    use crate::Deck;
    use crate::cards::Card;
    use crate::evaluator::{eval_5cards, hand_rank};
    
    #[test]
    fn test_evaluator(){
        let mut frequencies: [u128; 9] = [0; 9];

        let deck = Deck::new();

        for a in 0..52{
            for b in (a+1)..52{
                for c in (b+1)..52{
                    for d in (c+1)..52 {
                        for e in (d+1)..52 {
                            let board = vec![Card::from_bit_value(deck.cards[a]), Card::from_bit_value(deck.cards[b]), Card::from_bit_value(deck.cards[c]), Card::from_bit_value(deck.cards[d]), Card::from_bit_value(deck.cards[e])];
                            
                            let score = eval_5cards(&board);
                            let rank = hand_rank(score);
                            frequencies[rank as usize] += 1;
                        }
                    }
                }
            }
        }

        assert_eq!(frequencies, [1302540, 1098240, 123552, 54912, 10200, 5108, 3744, 624, 40]);
    }
}