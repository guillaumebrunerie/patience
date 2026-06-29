use crate::{card::Card, configuration, deck::Deck};

#[derive(Default)]
pub struct Foundations {
    hearts: Vec<Card>,
    spades: Vec<Card>,
    diamonds: Vec<Card>,
    clubs: Vec<Card>,
}

pub struct SidedCard {
    pub card: Card,
    pub is_hidden: bool,
}

pub struct State {
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: Foundations,
    pub tableau: Vec<Vec<SidedCard>>,
}

impl State {
    pub fn new(configuration: &configuration::Configuration) -> Result<State, String> {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut tableau = Vec::new();
        for i in 0..configuration.tableau_size {
            let mut pile = Vec::new();
            for j in 0..i + 1 {
                pile.push(SidedCard {
                    card: deck.pop().ok_or("Not enough cards in deck".to_string())?,
                    is_hidden: j < i,
                });
            }
            tableau.push(pile);
        }
        Ok(State {
            stock: deck.cards,
            waste: Vec::new(),
            foundations: Foundations::default(),
            tableau,
        })
    }

    pub fn deal(&mut self, card_count: u16) {
        for _ in 0..card_count {
            match self.stock.pop() {
                Some(card) => self.waste.push(card),
                None => return,
            }
        }
    }

    pub fn display(&self) {}
}
