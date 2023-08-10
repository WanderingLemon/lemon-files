use lemon_files::app::{App, AppResult};
use lemon_files::event::{Event, EventHandler};
use lemon_files::handler::handle_key_events;
use lemon_files::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

// }fn main() {
//
//     let dir = fs::read_dir(".").unwrap();
//     for entry in dir {
//         let entry = entry.unwrap();
//         let meta = entry.metadata().unwrap();
//         let permissions = meta.permissions().mode();
//
//         println!("{}", String::from(entry.path().to_string_lossy().strip_prefix("./").unwrap()));
//     }
// }
