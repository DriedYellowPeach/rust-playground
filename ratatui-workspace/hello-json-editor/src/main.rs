pub mod app;
pub mod ui;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{CrosstermBackend, Backend};
use ratatui::Terminal;
use std::io;

use app::{App, CurrentScreen, CurrentlyEditing};
use ui::ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    // we are using stderr because we want stdout to be redirect by user
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    match res {
        Ok(do_print) => {
            if do_print {
                app.print_json()?;
            }
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(app::CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }
                CurrentScreen::Exiting => {
                    match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') => {
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                CurrentScreen::Editing if key.kind == event::KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_kv();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => app.key_input.pop(),
                                    CurrentlyEditing::Value => app.value_input.pop(),
                                };
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
