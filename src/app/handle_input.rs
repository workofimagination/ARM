use crate::app::{App, Mode, Event};
use crate::driver::{self, DriverError};

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::disable_raw_mode;
use std::sync::mpsc::Receiver;

use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::Terminal;

impl App {
pub fn handle_input(&mut self, rx: &Receiver<Event<KeyEvent>>, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    match rx.recv().unwrap() {
        Event::Input(event) => match event.code {
            event => match self.current_mode {
                Mode::Normal => match event {
                    KeyCode::Esc => {
                        self.current_mode = Mode::Normal 
                    },

                    KeyCode::Char('c') => { self.current_mode = Mode::Control }

                    KeyCode::Char(':') => { self.current_mode = Mode::Buffer }

                    KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        terminal.show_cursor().unwrap();
                        self.save_current_angles();
                        std::process::exit(0)
                    },

                    KeyCode::Char('d') => { dbg!(self.get_2d_points()); }

                    KeyCode::Char('a') => { self.add_random_point(); },

                        KeyCode::Char('s') => { self.save_current_angles(); },

                        KeyCode::Char('p') => { self.flush_prev_positions(); },

                        KeyCode::Char('f') => { self.flush_command_output(); },

                        KeyCode::Char('=') => { self.increase_prev_points(); },

                        KeyCode::Char('-') => { self.decrease_prev_points(); },

                        KeyCode::Char(']') => { self.increase_command_ouput(); },

                        KeyCode::Char('[') => { self.decrease_command_output(); }

                        _ => {}
                    },

                    Mode::Control => match event {
                        KeyCode::Esc => { self.current_mode = Mode::Normal},

                        KeyCode::Left => { self.move_direction(driver::Direction::Left); },

                        KeyCode::Right => { self.move_direction(driver::Direction::Right); },

                        KeyCode::Up => { self.move_direction(driver::Direction::Up); },

                        KeyCode::Down => { self.move_direction(driver::Direction::Down); },

                        KeyCode::Char('\\') => { self.goto_smooth(); }

                        KeyCode::Char('=') => { self.increase_movement_amount(); },

                        KeyCode::Char('-') => { self.decrease_movement_amount(); }

                        KeyCode::Char('[') => { self.decrease_max_delay(); }

                        KeyCode::Char(']') => { self.increase_max_delay(); }

                        KeyCode::Char(';') => { self.decrease_min_delay(); }

                        KeyCode::Char('\'') => { self.increase_min_delay(); }

                        KeyCode::Char(',') => { self.decrease_delay(); }

                        KeyCode::Char('.') => { self.increase_delay(); }

                        KeyCode::Enter => { self.goto(); }

                        _ => {}
                    },

                    Mode::Buffer => match event {
                        KeyCode::Esc => { self.current_mode = Mode::Normal; },

                        KeyCode::Enter => { self.current_mode = Mode::Normal; }

                        KeyCode::Char(c) => { self.buffer.push(c); },

                        KeyCode::Backspace => { self.buffer.pop(); },

                        KeyCode::Delete => { self.buffer.clear(); }

                        _ => {}
                    }
                }
            },

            Event::Tick => {}
        }
    }

}
