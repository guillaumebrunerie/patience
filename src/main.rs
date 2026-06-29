use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};
use patience::{configuration, drawing, game::State, ui_state::UiState};

fn main() -> Result<(), Box<dyn Error>> {
    let configuration = configuration::Configuration {
        tableau_size: 7,
        waste_cards_to_show: 3,
        suits: 4,
    };

    let mut state = State::new(&configuration)?;
    let mut ui_state = UiState::new(&configuration);

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

    loop {
        drawing::draw_state(&mut stdout, &state, &ui_state, &configuration)?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(' ') => state.deal(1),
                KeyCode::Up => ui_state.move_up(),
                KeyCode::Down => ui_state.move_down(),
                KeyCode::Left => ui_state.move_left(),
                KeyCode::Right => ui_state.move_right(),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                KeyCode::Char('q') => break,
                _ => (),
            },
            _ => (),
        }
    }

    crossterm::execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
