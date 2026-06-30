use std::iter::repeat_with;

use crate::{card::Card, configuration::Configuration, deck::Deck};

pub struct SidedCard {
    pub card: Card,
    pub is_hidden: bool,
}

pub struct State<'a> {
    cfg: &'a Configuration,
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: Vec<Vec<Card>>,
    pub tableau: Vec<Vec<SidedCard>>,
}

impl<'a> State<'a> {
    pub fn new(cfg: &'a Configuration) -> Result<Self, String> {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut tableau = Vec::new();
        for i in 0..cfg.tableau_size {
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
            cfg,
            stock: deck.cards,
            waste: Vec::new(),
            foundations: repeat_with(Vec::new).take(cfg.suits as usize).collect(),
            tableau,
        })
    }

    pub fn max_counts(&self) -> Vec<u8> {
        self.tableau
            .iter()
            .map(|t| t.iter().filter(|c| !c.is_hidden).count() as u8)
            .collect()
    }

    fn deal(&mut self, card_count: u8) {
        if self.stock.len() == 0 {
            while let Some(card) = self.waste.pop() {
                self.stock.push(card);
            }
        } else {
            for _ in 0..card_count {
                match self.stock.pop() {
                    Some(card) => self.waste.push(card),
                    None => return,
                }
            }
        }
    }

    pub fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Deal => self.deal(self.cfg.cards_to_deal),
            Command::Move(move_from, move_to) => {
                let cards: Vec<Card> = match move_from {
                    MoveFrom::Waste => self.waste.pop().into_iter().collect(),
                    MoveFrom::Foundation(i) => {
                        self.foundations[i as usize].pop().into_iter().collect()
                    }
                    MoveFrom::Tableau { index, count } => {
                        let column = &mut self.tableau[index as usize];
                        column
                            .split_off(column.len() - count as usize)
                            .into_iter()
                            .map(|c| c.card)
                            .collect()
                    }
                };
                match move_to {
                    MoveTo::Foundation(i) => self.foundations[i as usize].extend(cards),
                    MoveTo::Tableau(i) => {
                        self.tableau[i as usize].extend(cards.into_iter().map(|card| SidedCard {
                            card,
                            is_hidden: false,
                        }))
                    }
                }
            }
        }
    }

    pub fn single_card_suit(&self, move_from: MoveFrom) -> Option<u8> {
        match move_from {
            MoveFrom::Waste => {
                let card = self.waste.get(self.waste.len() - 1)?;
                Some(card.suit as u8)
            }
            MoveFrom::Foundation(i) => Some(i),
            MoveFrom::Tableau { index, count } => {
                let column = &self.tableau[index as usize];
                let card = column.get(column.len() - 1)?;
                if count == 1 && !card.is_hidden {
                    Some(card.card.suit as u8)
                } else {
                    None
                }
            }
        }
    }
}

pub enum MoveFrom {
    Waste,
    Foundation(u8),
    Tableau { index: u8, count: u8 },
}

pub enum MoveTo {
    Foundation(u8),
    Tableau(u8),
}

pub enum Command {
    Deal,
    Move(MoveFrom, MoveTo),
}
