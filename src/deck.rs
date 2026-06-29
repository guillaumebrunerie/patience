use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit_i in 0..=3 {
            for rank_j in 1..=13 {
                let card = Card {
                    suit: Suit::from_value(suit_i),
                    rank: Rank::from_value(rank_j),
                };
                cards.push(card);
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl IntoIterator for Deck {
    type Item = Card;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}
