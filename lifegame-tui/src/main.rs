use lifegame_tui::{
    event::{Event, EventHandler},
    game::{Game, GameResult, GameState},
    input::handle_key_input,
    tui::Tui,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> GameResult<()> {
    // Create an application.
    let mut app = Game::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.state != GameState::Quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_input(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
