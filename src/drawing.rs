use std::{error::Error, io::Write};

use crossterm::{
    cursor,
    style::{self, Color},
};

use crate::{
    card::{Card, Suit},
    configuration, game,
    ui_state::{self, Slot},
};

struct Cursor<'a, W: Write> {
    out: &'a mut W,
    x: u16,
    y: u16,
}

const WIDTH: u16 = 8;
const HEIGHT: u16 = 7;

impl<W: Write> Cursor<'_, W> {
    fn apply(&mut self) -> Result<(), Box<dyn Error>> {
        crossterm::queue!(self.out, cursor::MoveTo(self.x, self.y))?;
        Ok(())
    }

    fn offset(&mut self, x: u16, y: u16) -> Result<(), Box<dyn Error>> {
        crossterm::queue!(self.out, cursor::MoveTo(self.x + x, self.y + y))?;
        Ok(())
    }

    pub fn draw_box(&mut self, w: u16, h: u16, color: Color) -> Result<(), Box<dyn Error>> {
        let _ = crossterm::queue!(self.out, style::SetForegroundColor(color));
        self.apply()?;
        let border_h = "─".repeat(w as usize);
        write!(self.out, "┌{border_h}┐")?;
        let mut y = 0;
        for _ in 1..=h - 1 {
            y += 1;
            self.offset(0, y)?;
            let empty = " ".repeat(w as usize);
            write!(self.out, "│{empty}│")?;
        }
        y += 1;
        self.offset(0, y)?;
        write!(self.out, "└{border_h}┘")?;
        Ok(())
    }

    fn draw_card_box(&mut self, color: Color) -> Result<(), Box<dyn Error>> {
        self.draw_box(WIDTH, HEIGHT, color)?;
        Ok(())
    }

    fn draw_empty_slot(&mut self, is_selected: bool) -> Result<(), Box<dyn Error>> {
        let color = if is_selected {
            Color::DarkBlue
        } else {
            Color::Black
        };
        self.draw_card_box(color)
    }

    fn draw_flipped_card(&mut self, is_selected: bool) -> Result<(), Box<dyn Error>> {
        let color = if is_selected {
            Color::DarkCyan
        } else {
            Color::DarkGrey
        };
        self.draw_card_box(color)
    }

    pub fn draw_card(&mut self, card: &Card, is_selected: bool) -> Result<(), Box<dyn Error>> {
        let color = if is_selected {
            Color::DarkBlue
        } else {
            Color::White
        };

        self.draw_card_box(color)?;

        let color = match card.suit {
            Suit::Hearts | Suit::Diamonds => Color::DarkRed,
            Suit::Spades | Suit::Clubs => Color::Grey,
        };
        crossterm::queue!(self.out, style::SetForegroundColor(color))?;

        self.offset(2, 1)?;
        write!(self.out, "{}", card.rank)?;

        self.offset(WIDTH - 1, 1)?;
        write!(self.out, "{}", card.suit)?;

        self.offset(2, HEIGHT - 1)?;
        write!(self.out, "{}", card.suit)?;

        self.offset(WIDTH - 1, HEIGHT - 1)?;
        write!(self.out, "{}", card.rank)?;

        Ok(())
    }
}

pub fn draw_state<W: Write>(
    out: &mut W,
    state: &game::State,
    ui_state: &ui_state::UiState,
    configuration: &configuration::Configuration,
) -> Result<(), Box<dyn Error>> {
    let x = 3;
    let y = 3;
    let spacing_x = 14;
    let spacing_y = 12;
    let offset_y = 2;
    let hidden_offset_y = 1;
    let offset_x = 4;

    let mut cursor = Cursor { out, x, y };

    // Stock
    if state.stock.len() > 0 {
        cursor.draw_flipped_card(ui_state.selected == Slot::Stock)?;
        cursor.offset(WIDTH / 2 - 1, HEIGHT / 2)?;
        write!(cursor.out, "({})", state.stock.len())?;
    } else {
        cursor.draw_empty_slot(ui_state.selected == Slot::Stock)?;
    }

    // Waste
    cursor.x += spacing_x;
    cursor.draw_empty_slot(ui_state.selected == Slot::Waste)?;
    let waste_len = state.waste.len() as u8;
    for i in 0..configuration.waste_cards_to_show {
        if waste_len + i >= configuration.waste_cards_to_show {
            let index = waste_len + i - configuration.waste_cards_to_show;
            cursor.draw_card(
                &state.waste[index as usize],
                ui_state.selected == Slot::Waste && index == waste_len - 1,
            )?;
            cursor.x += offset_x;
        }
    }

    // Foundations
    cursor.x = x + spacing_x * 3;
    for i in 0..configuration.suits {
        cursor.draw_empty_slot(ui_state.selected == Slot::Foundation(i))?;
        cursor.x += spacing_x;
    }

    // Tableau
    cursor.x = x;
    let mut i = 0;
    for column in &state.tableau {
        cursor.y = y + spacing_y;
        for game::SidedCard { is_hidden, card } in column {
            if *is_hidden {
                cursor.draw_flipped_card(false)?;
                cursor.y += hidden_offset_y;
            } else {
                cursor.draw_card(card, ui_state.selected == Slot::Tableau(i))?;
                cursor.y += offset_y;
            }
        }
        cursor.x += spacing_x;
        i += 1;
    }
    Ok(())
}
