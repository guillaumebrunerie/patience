use crate::configuration;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Slot {
    Stock,
    Waste,
    Foundation(u8),
    Tableau(u8),
}

impl Slot {
    pub fn up(self) -> Self {
        match self {
            Slot::Tableau(0) => Slot::Stock,
            Slot::Tableau(1) => Slot::Waste,
            Slot::Tableau(2) => Slot::Waste,
            Slot::Tableau(n) => Slot::Foundation(n - 3),
            s => s,
        }
    }

    pub fn down(self) -> Self {
        match self {
            Slot::Stock => Slot::Tableau(0),
            Slot::Waste => Slot::Tableau(1),
            Slot::Foundation(n) => Slot::Tableau(n + 3),
            s => s,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Slot::Waste => Slot::Stock,
            Slot::Foundation(0) => Slot::Waste,
            Slot::Foundation(n) => Slot::Foundation(n - 1),
            Slot::Tableau(0) => Slot::Tableau(0),
            Slot::Tableau(n) => Slot::Tableau(n - 1),
            s => s,
        }
    }

    pub fn right(self) -> Self {
        match self {
            Slot::Stock => Slot::Waste,
            Slot::Waste => Slot::Foundation(0),
            Slot::Foundation(3) => Slot::Foundation(3),
            Slot::Foundation(n) => Slot::Foundation(n + 1),
            Slot::Tableau(6) => Slot::Tableau(6),
            Slot::Tableau(n) => Slot::Tableau(n + 1),
        }
    }
}

pub struct UiState<'a> {
    configuration: &'a configuration::Configuration,
    pub selected: Slot,
}

impl<'a> UiState<'a> {
    pub fn new(configuration: &'a configuration::Configuration) -> Self {
        UiState {
            configuration,
            selected: Slot::Stock,
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.up();
    }

    pub fn move_down(&mut self) {
        self.selected = self.selected.down();
    }

    pub fn move_left(&mut self) {
        self.selected = self.selected.left();
    }

    pub fn move_right(&mut self) {
        self.selected = self.selected.right();
    }
}
