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
    pub fn new() -> App {
        let shifting_vec_size = 10;

        let default_angleset = AngleSet { column_angle: 0.0, beam_angle: 0.0, rotation_angle: 0.0 };
        let prev_positions = ShiftingVec::<AngleSet>::initalize(shifting_vec_size, default_angleset);
        let prev_positions_size = 10;

        let default_output = String::from("");
        let command_output = ShiftingVec::<String>::initalize(shifting_vec_size, default_output);
        let command_output_size = 10;

        let current_mode = Mode::Normal;
        let buffer = String::from("");
        let driver = Driver::new();

        return App { prev_positions, command_output, current_mode, buffer, driver, prev_positions_size,
            command_output_size }
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
                let chunks = App::make_chunk(
                    Direction::Vertical,
                    vec![
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3)
                    ]
                ).split(size);

                let top_chunks = App::make_chunk(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(16),
                        Constraint::Percentage(17),
                        Constraint::Percentage(17),
                        Constraint::Percentage(17),
                        Constraint::Percentage(17),
                        Constraint::Percentage(16)
                    ]
                ).split(chunks[0]);


                let middle_chunks = App::make_chunk(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(40),
                        Constraint::Percentage(60),

                    ]
                ).split(chunks[1]);

                let middle_right_chunks = App::make_chunk(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                ).split(middle_chunks[1]);

                let middle_right_top_chunks = App::make_chunk(
                    Direction::Vertical,
                    vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                ).split(middle_right_chunks[0]);

                let middle_right_bottom_chunks = App::make_chunk(
                    Direction::Vertical,
                    vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                ).split(middle_right_chunks[1]);

                let middle_left_chunks = App::make_chunk(
                    Direction::Vertical, 
                    vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                ).split(middle_chunks[0]);

                let top_middle_left_chunks = App::make_chunk(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                ).split(middle_left_chunks[0]);

                let bottom_chunks = App::make_chunk(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(20),
                        Constraint::Percentage(80)
                    ]
                ).split(chunks[2]);

                let current_x = self.make_plain_paragraph(format!("Current X: {}", self.get_current_x()));
                rect.render_widget(current_x, top_chunks[0]);

                let current_y = self.make_plain_paragraph(format!("Current Y: {}", self.get_current_y()));
                rect.render_widget(current_y, top_chunks[1]);

                let current_z = self.make_plain_paragraph(format!("Current Z: {}", self.get_current_z()));
                rect.render_widget(current_z, top_chunks[2]);

                let current_column = self.make_plain_paragraph(format!("Column Angle: {}", self.get_current_column_angle()));
                rect.render_widget(current_column, top_chunks[3]);

                let current_beam = self.make_plain_paragraph(format!("Beam Angle: {}", self.get_current_beam_angle()));
                rect.render_widget(current_beam, top_chunks[4]);

                let current_base = self.make_plain_paragraph(format!("Base Angle: {}", self.get_current_base_angle()));
                rect.render_widget(current_base, top_chunks[5]);

                let buffer = self.make_buffer();
                rect.render_widget(buffer, bottom_chunks[1]);

                
                // COME BACK HERE
                let data = self.get_2d_points();
                let map = self.make_map(&data, String::from("X-Y"), [-2.0, 2.0], [0.0, 2.0]);
                rect.render_widget(map, middle_right_bottom_chunks[1]);

                let x_z_data = self.get_x_z_points();
                let x_z_map = self.make_map(&x_z_data, String::from("X-Z"), [0.0, 2.0], [-2.0, 2.0]);
                rect.render_widget(x_z_map, middle_right_top_chunks[0]);

                //dude the naming the for the middle right chunks if fucked up
                let true_x_y_data = vec![(self.driver.current_position.x as f64, self.driver.current_position.y as f64)];
                let true_x_y_map = self.make_map(&true_x_y_data, String::from("True X-Y"), [0.0, 2.0], [0.0, 2.0]);
                rect.render_widget(true_x_y_map, middle_right_top_chunks[1]);

                let true_x_z_data = vec![(self.driver.current_position.x as f64, self.driver.current_position.z as f64)];
                let true_x_z_map = self.make_map(&true_x_z_data, String::from("True X-Z"), [0.0, 2.0], [-2.0, 2.0]);
                rect.render_widget(true_x_z_map, middle_right_bottom_chunks[0]);

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
