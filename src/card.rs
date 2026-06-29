use std::fmt::Display;

use crossterm::style::Stylize;
use rand::RngExt;

pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn from_value(n: u8) -> Self {
        match n {
            0 => Suit::Hearts,
            1 => Suit::Spades,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            _ => panic!("Invalid value {n}"),
        }
    }

    fn fmt_with_color(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        value: impl Display,
    ) -> std::fmt::Result {
        match self {
            Suit::Hearts | Suit::Diamonds => write!(f, "{}", format!("{value}").red()),
            Suit::Spades | Suit::Clubs => write!(f, "{value}"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        };
        write!(f, "{}", s,)
    }
}

pub enum Rank {
    A,
    N(u8), // 2 -> 9
    T,     // Ten
    J,
    Q,
    K,
}

impl Rank {
    pub fn from_value(n: u8) -> Self {
        match n {
            1 => Rank::A,
            n if n >= 2 && n <= 9 => Rank::N(n),
            10 => Rank::T,
            11 => Rank::J,
            12 => Rank::Q,
            13 => Rank::K,
            _ => panic!("Invalid value {n}"),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rank::A => "A",
            Rank::N(n) => &n.to_string(),
            Rank::T => "T",
            Rank::J => "J",
            Rank::Q => "Q",
            Rank::K => "K",
        };
        write!(f, "{}", s,)
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Card { suit, rank } = self;
        suit.fmt_with_color(f, format_args!("{rank} {suit}"))
    }
}

impl Card {
    pub fn pick() -> Card {
        let mut rng = rand::rng();

        let suit = Suit::from_value(rng.random_range(0..=3));
        let rank = Rank::from_value(rng.random_range(1..=13));
        Card { suit, rank }
    }
}
