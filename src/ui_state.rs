use std::cmp::min;

use crate::{
    configuration::Configuration,
    game::{self, Command, MoveFrom, MoveTo},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Selection {
    Stock,
    Waste,
    Foundation(u8),
    Tableau { index: u8, count: u8 },
}

impl Selection {
    pub fn up(self, cfg: &Configuration) -> Self {
        match self {
            Selection::Tableau { index: 0, .. } => Selection::Stock,
            Selection::Tableau { index: 1 | 2, .. } => Selection::Waste,
            Selection::Tableau { index, .. } => {
                Selection::Foundation(min(index - 3, cfg.suits - 1))
            }
            s => s,
        }
    }

    pub fn down(self, cfg: &Configuration) -> Self {
        match self {
            Selection::Stock => Selection::Tableau { index: 0, count: 1 },
            Selection::Waste => Selection::Tableau { index: 1, count: 1 },
            Selection::Foundation(n) => Selection::Tableau {
                index: min(n + 3, cfg.tableau_size - 1),
                count: 1,
            },
            s => s,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Selection::Waste => Selection::Stock,
            Selection::Foundation(0) => Selection::Waste,
            Selection::Foundation(n) => Selection::Foundation(n - 1),
            Selection::Tableau { index: 0, .. } => self,
            Selection::Tableau { index: n, .. } => Selection::Tableau {
                index: n - 1,
                count: 1,
            },
            s => s,
        }
    }

    pub fn right(self, cfg: &Configuration) -> Self {
        match self {
            Selection::Stock => Selection::Waste,
            Selection::Waste => Selection::Foundation(0),
            Selection::Foundation(n) => Selection::Foundation(min(n + 1, cfg.suits - 1)),
            Selection::Tableau { index, .. } if index == cfg.tableau_size - 1 => self,
            Selection::Tableau { index, .. } => Selection::Tableau {
                index: index + 1,
                count: 1,
            },
        }
    }

    pub fn increase(self, max_counts: &[u8]) -> Self {
        match self {
            Selection::Tableau { index, count } => Selection::Tableau {
                index,
                count: min(count + 1, max_counts[index as usize]),
            },
            s => s,
        }
    }

    pub fn decrease(self, cfg: &Configuration, max_counts: &[u8]) -> Self {
        match self {
            Selection::Tableau { index, count: 1 } => Selection::Tableau {
                index,
                count: max_counts[index as usize],
            },
            Selection::Tableau { index, count } => Selection::Tableau {
                index,
                count: count - 1,
            },
            s => s.down(cfg),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Target {
    Foundation(u8),
    Tableau(u8),
}

impl Target {
    pub fn up(self, selected_card_suit: Option<u8>) -> Self {
        match self {
            Target::Tableau(_) if let Some(i) = selected_card_suit => Target::Foundation(i),
            s => s,
        }
    }

    pub fn down(self, cfg: &Configuration) -> Self {
        match self {
            Target::Foundation(i) => Target::Tableau(min(i + 3, cfg.tableau_size - 1)),
            s => s,
        }
    }

    pub fn left(self, cfg: &Configuration) -> Self {
        match self {
            Target::Tableau(n) => Target::Tableau((n - 1).clamp(0, cfg.tableau_size - 1)),
            s => s,
        }
    }

    pub fn right(self, cfg: &Configuration) -> Self {
        match self {
            Target::Tableau(n) => Target::Tableau((n + 1).clamp(0, cfg.tableau_size - 1)),
            s => s,
        }
    }
}

pub struct UiState<'a> {
    cfg: &'a Configuration,
    pub selected: Selection,
    pub target: Option<Target>,
}

impl<'a> UiState<'a> {
    pub fn new(cfg: &'a Configuration) -> Self {
        UiState {
            cfg,
            selected: Selection::Stock,
            target: None,
        }
    }

    pub fn move_up(&mut self, selected_card_suit: Option<u8>) {
        match self.target {
            None => self.selected = self.selected.up(self.cfg),
            Some(target) => self.target = Some(target.up(selected_card_suit)),
        }
    }

    pub fn move_down(&mut self) {
        match self.target {
            None => self.selected = self.selected.down(self.cfg),
            Some(target) => self.target = Some(target.down(self.cfg)),
        }
    }

    pub fn move_left(&mut self) {
        match self.target {
            None => self.selected = self.selected.left(),
            Some(target) => self.target = Some(target.left(self.cfg)),
        }
    }

    pub fn move_right(&mut self) {
        match self.target {
            None => self.selected = self.selected.right(self.cfg),
            Some(target) => self.target = Some(target.right(self.cfg)),
        }
    }

    pub fn increase_selection(&mut self, max_counts: &[u8]) {
        if self.target.is_none() {
            self.selected = self.selected.increase(max_counts)
        }
    }

    pub fn decrease_selection(&mut self, max_counts: &[u8]) {
        if self.target.is_none() {
            self.selected = self.selected.decrease(self.cfg, max_counts)
        }
    }

    pub fn activate(&mut self) -> Option<Command> {
        match self.target {
            Some(_) => Some(Command::Move(self.to_move_from(), self.to_move_to())),
            None => match self.selected {
                Selection::Stock => Some(Command::Deal),
                Selection::Tableau { index, .. } => {
                    self.target = Some(Target::Tableau(index));
                    None
                }
                _ => None,
            },
        }
    }

    pub fn deactivate(&mut self) {
        self.target = None;
    }

    pub fn to_move_from(&self) -> game::MoveFrom {
        match self.selected {
            Selection::Stock => MoveFrom::Waste,
            Selection::Waste => MoveFrom::Waste,
            Selection::Foundation(i) => MoveFrom::Foundation(i),
            Selection::Tableau { index, count } => MoveFrom::Tableau { index, count },
        }
    }

    pub fn to_move_to(&self) -> game::MoveTo {
        match self.target {
            Some(Target::Foundation(i)) => MoveTo::Foundation(i),
            Some(Target::Tableau(i)) => MoveTo::Tableau(i),
            None => panic!(),
        }
    }
}
