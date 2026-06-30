use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, ClearType},
};
use patience::{configuration::Configuration, drawing, game::State, ui_state::UiState};

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Configuration {
        tableau_size: 7,
        waste_cards_to_show: 3,
        cards_to_deal: 3,
        suits: 4,
    };

    let mut state = State::new(&cfg)?;
    let mut ui_state = UiState::new(&cfg);

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

    loop {
        crossterm::queue!(stdout, terminal::Clear(ClearType::All))?;
        drawing::draw_state(&mut stdout, &state, &ui_state, &cfg)?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(' ') => {
                    if let Some(cmd) = ui_state.activate() {
                        state.apply(cmd)
                    }
                }
                KeyCode::Esc => ui_state.deactivate(),
                KeyCode::Up if key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                    ui_state.increase_selection(&state.max_counts())
                }
                KeyCode::Up => ui_state.move_up(state.single_card_suit(ui_state.to_move_from())),
                KeyCode::Down if key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                    ui_state.decrease_selection(&state.max_counts())
                }
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
