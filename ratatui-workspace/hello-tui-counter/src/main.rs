use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)),
        f.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(k) = event::read()? {
            if k.kind == event::KeyEventKind::Press {
                match k.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        t.draw(|f| {
            ui(&app, f);
        })?;

        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    Ok(())

}

fn main() -> Result<()> {
    startup()?;
    let res = run();
    shutdown()?;
    res?;
    Ok(())
}
