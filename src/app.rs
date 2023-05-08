use crate::driver::{self, DriverError};
use crate::driver::Driver;
use crate::utils::{ShiftingVec, Utils};
use crate::Calc;

use std::num::ParseFloatError;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;

use crossterm::event::{self, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{Terminal, symbols};
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Block, Borders, ListItem, List, Dataset, Chart, Axis, GraphType};

enum Event<I> {
    Input(I),
    Tick
}

enum Mode {
    Normal,
    Safe,
    Control,
    Buffer
}

pub struct Point {
    x: f32,
    y: f32,
    column_angle: f32,
    beam_angle: f32
}

pub struct AngleSet {
    column_angle: f32,
    beam_angle: f32,
    rotation_angle: f32
}

pub struct App {
    prev_positions: ShiftingVec<Point>,
    command_output: ShiftingVec<String>,
    current_mode: Mode,
    buffer: String,
    driver: Driver,
}

impl App {
    pub fn make() -> App {
        let prev_positions = ShiftingVec::<Point>::initalize(12);
        let command_output = ShiftingVec::<String>::initalize(12);

        let current_mode = Mode::Safe;

        let buffer = String::from("");

        let driver = Driver::new();

        return App { prev_positions, command_output, current_mode, buffer, driver }

    }

    pub fn start(&mut self) {
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
                    .margin(2)
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

                let buffer = Paragraph::new(self.buffer.clone())
                    .style(Style::default())
                    .alignment(tui::layout::Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .border_type(tui::widgets::BorderType::Plain)
                    );
                rect.render_widget(buffer, bottom_chunks[1]);

                let data = self.get_2d_points();
                //let dataset = self.get_datasets();
                let dataset = vec![
                    Dataset::default()
                        .marker(symbols::Marker::Braille)
                        .graph_type(GraphType::Line)
                        .data(&data)
                ];
                
                let map = Chart::new(dataset)
                    .block(
                        Block::default()
                        .title("map")
                        .borders(Borders::ALL)
                    )
                    .x_axis(
                        Axis::default()
                        .bounds([-2.0, 2.0])
                    )
                    .y_axis(
                        Axis::default()
                        .bounds([0.0, 2.0])
                    );
                
                rect.render_widget(map, middle_chunks[1]);

                let config_data = self.get_config_text();
                let config = Paragraph::new(config_data)
                    .block(
                        Block::default()
                            .title("Config")
                            .borders(Borders::ALL)
                    );

                rect.render_widget(config, top_middle_left_chunks[1]);

                let items: Vec<ListItem> = self.prev_positions.get_items()
                    .iter()
                    .map(|i| {
                        let content = Spans::from(Span::styled(
                            format!("{} {}", i.column_angle, i.beam_angle),
                            Style::default() 
                        ));

                        ListItem::new(content).style(Style::default())
                    })
                    .collect();

                let prev_items = List::new(items)
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .title("Previous Positions")
                    );

                rect.render_stateful_widget(prev_items, top_middle_left_chunks[0], &mut self.prev_positions.get_state());

                let command_items: Vec<ListItem> = self.command_output.get_items()
                    .iter()
                    .map(|i| {
                        let content = Spans::from(Span::styled(
                            format!("{}", i),
                            Style::default() 
                        ));

                        ListItem::new(content).style(Style::default())
                    })
                    .collect();

                let command_items = List::new(command_items)
                    .block(
                        Block::default()
                            .title("Command Output")
                            .borders(Borders::ALL)
                    );

                rect.render_stateful_widget(command_items, middle_left_chunks[1], &mut self.command_output.get_state());

                let current_mode_box = Paragraph::new(self.get_current_mode_string())
                    .style(Style::default())
                    .alignment(tui::layout::Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .border_type(tui::widgets::BorderType::Plain)
                    );
                rect.render_widget(current_mode_box, bottom_chunks[0]);
            }).unwrap();

            // END OF PAGE RENDERING, MOVE THIS SHIT INTO A FUNCTION LATER

            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    event => match self.current_mode {
                        Mode::Normal => match event {
                            KeyCode::Esc => {
                                self.current_mode = Mode::Safe 
                            },

                            KeyCode::Char('q') => {
                                disable_raw_mode().unwrap();
                                terminal.show_cursor().unwrap();
                                self.save_current_angles();
                                break;
                            },

                            KeyCode::Char('d') => {
                                dbg!(self.get_2d_points());
                            }

                            KeyCode::Char('a') => {
                                self.add_random_point();
                            },

                            KeyCode::Char('s') => {
                                self.save_current_angles();
                            },

                            _ => {}
                        },

                        Mode::Safe => match event {
                            KeyCode::Char('q') => {
                                disable_raw_mode().unwrap();
                                terminal.show_cursor().unwrap();
                                self.save_current_angles();
                                break;
                            },

                            KeyCode::Char('n') => {
                                self.current_mode = Mode::Normal 
                            },

                            KeyCode::Char('c') => {
                                self.current_mode = Mode::Control
                            }

                            KeyCode::Char(':') => {
                                self.current_mode = Mode::Buffer
                            }

                            _ => {}
                        },

                        Mode::Control => match event {
                            KeyCode::Esc => { self.current_mode = Mode::Safe },

                            KeyCode::Left => { self.move_direction(driver::Direction::Left) },

                            KeyCode::Right => { self.move_direction(driver::Direction::Right); },

                            KeyCode::Up => { self.move_direction(driver::Direction::Up); },

                            KeyCode::Down => { self.move_direction(driver::Direction::Down); },

                            KeyCode::Char('\\') => { self.goto_smooth(); }

                            KeyCode::Char('+') => { self.increase_movement_amount(); },

                            KeyCode::Char('-') => { self.decrease_movement_amount(); }

                            KeyCode::Char('[') => { self.increase_max_delay(); }

                            KeyCode::Char(']') => { self.decrease_max_delay(); }

                            KeyCode::Char(';') => { self.increase_min_delay(); }

                            KeyCode::Char('\'') => { self.decrease_min_delay(); }

                            KeyCode::Char(',') => { self.decrease_delay(); }

                            KeyCode::Char('.') => { self.increase_delay(); }

                            KeyCode::Enter => { self.goto(); }

                            _ => {}
                        },

                        Mode::Buffer => match event {
                            KeyCode::Esc => {
                                self.current_mode = Mode::Safe
                            },

                            KeyCode::Enter => {
                                self.current_mode = Mode::Safe
                            }

                            KeyCode::Char(c) => {
                                self.buffer.push(c);
                            },

                            KeyCode::Backspace => {
                                self.buffer.pop();
                            },

                            KeyCode::Delete => {
                                self.buffer.clear();
                            }

                            _ => {}
                        }
                    }
                },

                Event::Tick => {}
            }
        }
    }

    fn gen_random_point() -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1.0..3.0);
        let y = rng.gen_range(1.0..3.0);
        let column_angle = rng.gen_range(1.0..3.0);
        let beam_angle = rng.gen_range(1.0..3.0);

        return Point { x, y, column_angle, beam_angle }
    }

    fn add_random_point(&mut self) {
        let rand = App::gen_random_point();
        self.prev_positions.insert(rand);
    }

    fn add_current_position(&mut self) {
        let current_point = self.driver.current_position.clone();
        let beam_angle = self.driver.get_beam_angle();
        let column_angle = self.driver.get_column_angle();

        let current_position = Point { x: current_point.x, y: current_point.y, beam_angle, column_angle };

        self.prev_positions.insert(current_position);
    }

    fn move_direction(&mut self, dir: driver::Direction) {
        match self.driver.move_direction(dir) {
            Ok(()) => (),
            Err(e) => match e {
                DriverError::UnReachable => {
                    let error_message = String::from("unable to reach position");
                    self.command_output.insert(error_message);
                }
            }
        }
    }

    fn goto_smooth(&mut self) {
        let (x, y) = match self.parse_buffer_goto() {
            Ok(x) => x,
            Err(e) => {
                self.command_output.insert(format!("{}", e));
                return
            }
        };

        self.add_current_position();
        
        match self.driver.goto_point_smooth(x, y) {
            Ok(()) => (),
            Err(e) => match e {
                DriverError::UnReachable => {
                    self.command_output.insert(format!("unable to reach location"));
                    return
                }
            }
        }
    }

    fn goto(&mut self) {
        let (x, y) = match self.parse_buffer_goto() {
            Ok(x) => x,
            Err(e) => {
                self.command_output.insert(format!("{}", e));
                return
            }
        };

        self.command_output.insert(format!("successfully parsed buffer"));

        self.add_current_position();

        match self.driver.goto_point(x, y) {
            Ok(()) => self.command_output.insert(format!("successfully wennt to point {} {}", x, y)),
            Err(e) => match e {
                DriverError::UnReachable => self.command_output.insert(format!("out of range"))
            }
        }
    }

    fn parse_buffer_goto(&self) -> Result<(f32, f32), ParseFloatError> {
        let coords = self.buffer.split("-").collect::<Vec<&str>>();

        let x = match coords[0].parse::<f32>() {
            Ok(x) => x,
            Err(e) => return Err(e)
        };

        let y = match coords[1].parse::<f32>() {
            Ok(y) => y,
            Err(e) => return Err(e)
        };

        Ok((x, y))
    }

    fn get_current_mode_string(&self) -> &str {
        let string = match self.current_mode {
            Mode::Normal => { "Normal" },
            Mode::Safe => { "Safe" },
            Mode::Control => { "Control" },
            Mode::Buffer => { "Buffer" }
        };

        return string
    } 

    fn get_current_x(&self) -> f32 {
        return self.driver.current_position.x
    }

    fn get_current_y(&self) -> f32 {
        return self.driver.current_position.y
    }

    fn get_current_column_angle(&self) -> f32 {
        return self.driver.get_column_angle()
    }

    fn get_current_beam_angle(&self) -> f32 {
        return self.driver.get_beam_angle()
    }

    fn save_current_angles(&mut self) {
        let current_angles = self.get_current_angle_set();

        let save_string = format!("{} {} {}", current_angles.column_angle, current_angles.beam_angle, current_angles.rotation_angle);

        match Utils::save_to_file("./output".to_string(), save_string) {
            Ok(x) => self.command_output.insert(x),
            Err(e) => self.command_output.insert(format!("unable to save to file: {}", e))
        }
    }

    fn get_current_angle_set(&self) ->  AngleSet {
        let beam_angle = self.get_current_beam_angle();
        let column_angle = self.get_current_column_angle();

        return AngleSet { column_angle, beam_angle, rotation_angle: 0.0 }
    }

    fn make_plain_paragraph(&self, content: String) -> Paragraph {
        let paragraph = Paragraph::new(content)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
            );

        return paragraph
    }

    fn get_2d_points(&self) -> Vec<(f64, f64)>{
        let column = self.driver.get_column_position();
        let beam = self.driver.get_beam_position();

        return vec![(0.0,0.0), column, beam]
    }

    fn increase_movement_amount(&mut self) {
        self.driver.movement_amount *= 1.25;
    }

    fn decrease_movement_amount(&mut self) {
        self.driver.movement_amount /= 1.25;
    }

    fn increase_max_delay(&mut self) {
        self.driver.micro_delay_max += 10;
    }

    fn decrease_max_delay(&mut self) {
        self.driver.micro_delay_max -= 10;
    }

    fn increase_min_delay(&mut self) {
        self.driver.micro_delay_min += 10;
    }

    fn decrease_min_delay(&mut self) {
        self.driver.micro_delay_min -= 10;
    }

    fn increase_delay(&mut self) {
        self.driver.micro_delay_default += 10;
    }

    fn decrease_delay(&mut self) {
        self.driver.micro_delay_default -= 10;
    }

    fn get_config_text(&self) -> Vec<Spans>{
        let text = vec![
            Spans::from(vec![
                Span::raw("STP DELY: "),
                Span::raw(format!("{}", self.driver.micro_delay_default))
            ]),
            Spans::from(vec![
                Span::raw("MAX DELY: "),
                Span::raw(format!("{}", self.driver.micro_delay_max))
            ]),
            Spans::from(vec![
                Span::raw("MIN DELY: "),
                Span::raw(format!("{}", self.driver.micro_delay_min))
            ]),
            Spans::from(vec![
                Span::raw("MVNT AMT: "),
                Span::raw(format!("{}", self.driver.movement_amount))
            ]),
        ];

        return text
    }
}
