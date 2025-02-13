use crate::app::{App, Events, ServiceView};

use crossterm::event::{self, Event, KeyCode};

use log::info;

pub fn handle_key_events(app: &mut App) -> Option<Events> {
    if let Event::Key(key) = event::read().expect("Error keyboard input") {
        if app.is_in_logs {
            return handle_logs_key_events(app, key);
        } else {
            return handle_services_key_events(app, key);
        }
    }
    None
}

fn handle_logs_key_events(app: &mut App, key: crossterm::event::KeyEvent) -> Option<Events> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Events::Quit),
        KeyCode::Down | KeyCode::Char('j') => {
            let logs_len = if let Some(logs_arc) = &app.logs {
                let logs_map = logs_arc.lock().unwrap();
                logs_map
                    .get(&app.selected_priority.unwrap_or(4))
                    .map(|logs| logs.len())
                    .unwrap_or(0)
            } else {
                0
            };

            if app.current_line < logs_len.saturating_sub(1) {
                app.current_line += 1;
            }
            None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_line > 0 {
                app.current_line -= 1;
            }
            None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if let Some(p) = app.selected_priority {
                if p > 1 {
                    app.set_priority(p - 1);
                    app.set_current_line(0);
                }
            }
            None
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if let Some(p) = app.selected_priority {
                if p < 7 {
                    app.set_priority(p + 1);
                    app.set_current_line(0);
                }
            }
            None
        }
        KeyCode::Char('c') => {
            app.set_init();
            None
        }
        KeyCode::Char('y') => {
            info!("i want to yank this");
            None
        }
        KeyCode::Char(key) if ('1'..='7').contains(&key) => {
            app.set_current_line(0);
            app.selected_priority = Some(key.to_digit(10).unwrap() as u8);
            None
        }
        _ => None,
    }
}

fn handle_services_key_events(app: &mut App, key: crossterm::event::KeyEvent) -> Option<Events> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Events::Quit),
        KeyCode::Down | KeyCode::Char('j') => {
            let services_len = match &app.services {
                Some((u, f)) => {
                    if app.selected_service_view == ServiceView::Units {
                        u.len()
                    } else {
                        f.len()
                    }
                }
                None => 0,
            };
            if app.current_line < services_len - 1 {
                app.current_line += 1;
            }
            None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_line > 0 {
                app.current_line -= 1;
            }
            None
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if let ServiceView::Units = app.selected_service_view {
                app.set_current_line(0);
                app.set_view(ServiceView::UnitFiles);
            }
            None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if let ServiceView::UnitFiles = app.selected_service_view {
                app.set_current_line(0);
                app.set_view(ServiceView::Units);
            }
            None
        }
        KeyCode::Enter => {
            if let Some((u, f)) = &app.services {
                match app.selected_service_view {
                    ServiceView::Units => {
                        info!("HIT ENTER FOR UNITS");
                        if let Some(service) = u.get(app.current_line) {
                            info!("SELECTED SERVICE NOW {:?}", service);
                            app.selected_service = Some(service.name.clone());
                            app.is_in_logs = true;
                        }
                    }
                    ServiceView::UnitFiles => {
                        info!("HIT ENTER FOR UNITSFILES");
                        if let Some(service) = f.get(app.current_line) {
                            info!("SELECTED SERVICE NOW {:?}", service);
                            app.selected_service = Some(service.name.clone());
                            app.is_in_logs = true;
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
}
