use crate::deck::Deck;
use crate::cards::Card;

struct Player{
    hand: Vec<Card>,
    stack: u128,
    bet: u128,
}

enum Action{
    Fold,
    Check,
    Call,
    Raise(u128),
    AllIn,
} 

#[derive(PartialEq)]
enum State{
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}
    

impl Player{
    fn new(stack: u128) -> Player{
        Player{
            hand: Vec::new(),
            stack,
            bet: 0,
        }
    }

    fn add_card(&mut self, card: Card){
        self.hand.push(card);
    }

    fn bet(&mut self, amount: u128) -> u128{
        self.bet += amount;
        self.stack -= amount;
        return amount;
    }

    fn all_in(&mut self) -> u128{
        self.bet(self.stack)
    }

}

struct Game{
    players: Vec<Player>,
    blind: u128,
    button: usize,
    state: State,
    pot: u128,
    ante: u128,
    round_bet: u128
}

impl Game {
    pub fn new(blind: u128, players: Vec<Player>) -> Game{
        Game{
            players,
            blind,
            button: 0,
            state: State::PreFlop,
            pot: 0,
            ante: 0,
            round_bet: 0,
        }
    }

    pub fn play(&mut self){
        let mut deck = Deck::new();

        for p in self.players.iter_mut(){
            p.add_card(deck.draw());
            p.add_card(deck.draw());
        }

        let current_player = (self.button + 3) % self.players.len();
        let sb_player = (self.button + 1) % self.players.len();
        let bb_player = (self.button + 2) % self.players.len();
        
        self.pot += self.players[sb_player].bet(self.blind);
        self.pot += self.players[bb_player].bet(2*self.blind);

        while self.state == State::PreFlop  {
            let bet = self.players[current_player].bet;
            self.players[current_player].bet(2*self.blind - bet);
        }

    }
}