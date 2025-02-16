use crate::app::{App, Events};

use crate::core::clipboard::copy_to_clipboard;
use crate::ui::ui::View;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use log::{info, warn};

pub fn handle_key_events(app: &mut App) -> Option<Events> {
    if let Event::Key(key) = event::read().expect("Error keyboard input") {
        if app.ui.is_in_logs {
            return handle_logs_key_events(app, key);
        } else {
            return handle_services_key_events(app, key);
        }
    }
    None
}

fn handle_logs_key_events(app: &mut App, key: KeyEvent) -> Option<Events> {
    let logs_len = if let Some(logs_arc) = &app.logs {
        let logs_map = logs_arc.lock().unwrap();
        logs_map
            .get(
                &app.ui
                    .selected_priority
                    .unwrap_or(app.config.options.initial_priority),
            )
            .map(|logs| logs.len())
            .unwrap_or(0)
    } else {
        0
    };

    let allow_actions = !app.ui.is_showing_line_in_modal && !app.ui.is_showing_help;

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Events::Quit),
        KeyCode::Char('?') => Some(Events::GetHelp),
        KeyCode::Char('K') => Some(Events::GetLineInModal),
        KeyCode::Char('y') => {
            info!("i want to yank this");
            warn!(
                "yanked value in logs: {:?}",
                app.ui
                    .get_log_message(&app)
                    .unwrap_or("Error yanking command".to_string())
            );
            copy_to_clipboard(
                app.ui
                    .get_log_message(&app)
                    .unwrap_or("Error yanking log message".to_string()),
            )
            .ok();
            None
        }
        _ => {
            if allow_actions {
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.ui.move_cursor_down(logs_len);
                        None
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.ui.move_cursor_up();
                        None
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if let Some(p) = app.ui.selected_priority {
                            if p > 1 {
                                app.ui.set_priority(p - 1);
                            }
                        }
                        None
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if let Some(p) = app.ui.selected_priority {
                            if p < 7 {
                                app.ui.set_priority(p + 1);
                            }
                        }
                        None
                    }
                    KeyCode::Char('c') => {
                        app.clear_logs();
                        app.ui.is_in_logs = false;
                        //app.ui.set_init_priority(); TODO
                        None
                    }
                    KeyCode::Char(key) if ('1'..='7').contains(&key) => {
                        app.ui.set_current_line(0);
                        app.ui.selected_priority = Some(key.to_digit(10).unwrap() as u8);
                        None
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}

fn handle_services_key_events(app: &mut App, key: crossterm::event::KeyEvent) -> Option<Events> {
    let services_len = match &app.services {
        Some((u, f)) => {
            if app.ui.view == View::ServiceUnits {
                u.len()
            } else {
                f.len()
            }
        }
        None => 0,
    };

    let allow_actions = !app.ui.is_showing_line_in_modal && !app.ui.is_showing_help;

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Events::Quit),
        KeyCode::Char('?') => Some(Events::GetHelp),
        KeyCode::Char('K') => Some(Events::GetLineInModal),
        _ => {
            if allow_actions {
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.ui.move_cursor_down(services_len);
                        None
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.ui.move_cursor_up();
                        None
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if app.ui.view == View::ServiceUnits {
                            app.ui.set_current_line(0);
                            app.ui.set_view(View::ServiceUnitFiles);
                        }
                        None
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        if app.ui.view == View::ServiceUnitFiles {
                            app.ui.set_current_line(0);
                            app.ui.set_view(View::ServiceUnits);
                        }
                        None
                    }
                    KeyCode::Enter => {
                        if let Some((u, f)) = &app.services {
                            match app.ui.view {
                                View::ServiceUnits => {
                                    info!("HIT ENTER FOR UNITS");
                                    if let Some(service) = u.get(app.ui.current_line) {
                                        info!("SELECTED SERVICE NOW {:?}", service);
                                        app.selected_service = Some(service.name.clone());
                                        app.ui.is_in_logs = true;
                                        app.ui.set_current_line(0);
                                    }
                                }
                                View::ServiceUnitFiles => {
                                    info!("HIT ENTER FOR UNITSFILES");
                                    if let Some(service) = f.get(app.ui.current_line) {
                                        info!("SELECTED SERVICE NOW {:?}", service);
                                        app.selected_service = Some(service.name.clone());
                                        app.ui.is_in_logs = true;
                                        app.ui.set_current_line(0);
                                    }
                                }
                            }
                            Some(Events::GetLogs)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}
