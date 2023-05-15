mod handle_input;
mod makes;
mod backend;

use crate::driver::Driver;
use crate::utils::ShiftingVec;

use crossterm::event;
use crossterm::terminal::enable_raw_mode;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};


use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint};

pub enum Event<I> {
    Input(I),
    Tick
}

pub enum Mode {
    Normal,
    Safe,
    Control,
    Buffer
}

#[derive(Clone)]
pub struct AngleSet {
    column_angle: f32,
    beam_angle: f32,
    rotation_angle: f32
}

pub struct App {
    prev_positions: ShiftingVec<AngleSet>,
    prev_positions_size: usize,
    command_output: ShiftingVec<String>,
    command_output_size: usize,
    current_mode: Mode,
    buffer: String,
    driver: Driver,
}

impl App {
    pub fn make() -> App {
        let shifting_vec_size = 10;

        let default_angleset = AngleSet { column_angle: 0.0, beam_angle: 0.0, rotation_angle: 0.0 };
        let prev_positions = ShiftingVec::<AngleSet>::initalize(shifting_vec_size, default_angleset);
        let prev_positions_size = 10;

        let default_output = String::from("");
        let command_output = ShiftingVec::<String>::initalize(shifting_vec_size, default_output);
        let command_output_size = 10;

        let current_mode = Mode::Safe;
        let buffer = String::from("");
        let driver = Driver::new();

        return App { prev_positions, command_output, current_mode, buffer, driver, prev_positions_size, command_output_size }
    }

    pub fn start(&mut self) {
        self.flush_prev_positions();

        enable_raw_mode().expect("cannot run in raw mode");

        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));
    
                if event::poll(timeout).expect("poll works") {
                    if let event::Event::Key(key) = event::read().expect("event reading does not works") {
                        tx.send(Event::Input(key)).expect("cannot send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }
            }
        });

        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(0),
                            Constraint::Length(3)
                        ].as_ref()
                    )
                    .split(size);
                
                let top_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                            Constraint::Percentage(25),
                            Constraint::Percentage(25)
                        ]
                        .as_ref()
                    )
                    .split(chunks[0]);

                let middle_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(70),
                        ]
                        .as_ref()
                    )
                    .split(chunks[1]);

                let middle_left_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50)
                        ]
                        .as_ref()
                    )
                    .split(middle_chunks[0]);

                let top_middle_left_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(50),
                            Constraint::Percentage(50)
                        ]
                        .as_ref()
                    )
                    .split(middle_left_chunks[0]);

                let bottom_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(20),
                            Constraint::Percentage(80)
                        ]
                        .as_ref()
                    )
                    .split(chunks[2]);

                let current_x = self.make_plain_paragraph(format!("Current X: {}", self.get_current_x()));
                rect.render_widget(current_x, top_chunks[0]);


                let current_y = self.make_plain_paragraph(format!("Current Y: {}", self.get_current_y()));
                rect.render_widget(current_y, top_chunks[1]);


                let current_column = self.make_plain_paragraph(format!("Column Angle: {}", self.get_current_column_angle()));
                rect.render_widget(current_column, top_chunks[2]);


                let current_beam = self.make_plain_paragraph(format!("Beam Angle: {}", self.get_current_beam_angle()));
                rect.render_widget(current_beam, top_chunks[3]);

                let buffer = self.make_buffer();
                rect.render_widget(buffer, bottom_chunks[1]);

                
                let data = self.get_2d_points();
                let map = self.make_map(&data);
                rect.render_widget(map, middle_chunks[1]);


                let config = self.make_config_window();
                rect.render_widget(config, top_middle_left_chunks[1]);


                let (prev_items, state) = self.make_previous_points();
                rect.render_stateful_widget(prev_items, top_middle_left_chunks[0], state);


                let (command_items, state) = self.make_command_output();
                rect.render_stateful_widget(command_items, middle_left_chunks[1], state);


                let current_mode_box = self.make_current_mode_box();
                rect.render_widget(current_mode_box, bottom_chunks[0]);

            }).unwrap();

            // END OF PAGE RENDERING

            self.handle_input(&rx, &mut terminal);
        }
    }
}
