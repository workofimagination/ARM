use crate::handler::Handler;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;

use crossterm::event::{self, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Direction, Constraint};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::canvas::Canvas;
use tui::widgets::{Paragraph, Block, Borders, ListState, ListItem, List};


enum Event<I> {
    Input(I),
    Tick
}

struct GonetoPosition<T> {
    state: ListState,
    items: Vec<T>
}

pub struct Point {
    x: f32,
    y: f32,
    column_angle: f32,
    beam_angle: f32
}

pub struct App {
    prev_positions: GonetoPosition<Point>,
}

impl App {
    pub fn make() -> App {
        let positions: Vec<Point> = Vec::new();

        let prev_positions: GonetoPosition<Point> = GonetoPosition {
            state: ListState::default(),
            items: positions
        };

        return App { prev_positions }

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

                let middle_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(70)
                        ]
                        .as_ref()
                    )
                    .split(chunks[1]);

                let copyright = Paragraph::new("this is a test")
                    .style(Style::default())
                    .alignment(tui::layout::Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .title("testbox")
                            .border_type(tui::widgets::BorderType::Plain)
                    );
                rect.render_widget(copyright, chunks[2]);


                let canvas = Canvas::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("map")
                    )
                    .paint(|_ctx| {
                        
                    })
                    .x_bounds([0.0, 100.0])
                    .y_bounds([0.0, 100.0]);
                    

                rect.render_widget(canvas, middle_chunks[1]);

                let items: Vec<ListItem> = self.prev_positions.items
                    .iter()
                    .map(|i| {
                        let content = Spans::from(Span::styled(
                            format!("{} {} {} {}", i.x, i.y, i.column_angle, i.beam_angle),
                            Style::default() 
                        ));

                        ListItem::new(content).style(Style::default())
                    })
                    .collect();

                let items = List::new(items)
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .title("Previous Positions")
                    )
                    .highlight_symbol(">> ");

                rect.render_stateful_widget(items, middle_chunks[0], &mut self.prev_positions.state);

                self.add_random_point();

            }).unwrap();


            // END OF PAGE RENDERING, MOVE THIS SHIT INTO A FUNCTION LATER

            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        terminal.show_cursor().unwrap();
                        break;
                    },
                    _ => {}
                },
                Event::Tick => {}
            }
        }


    }

    pub fn gen_random_point() -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1.0..3.0);
        let y = rng.gen_range(1.0..3.0);
        let column_angle = rng.gen_range(1.0..3.0);
        let beam_angle = rng.gen_range(1.0..3.0);

        return Point { x, y, column_angle, beam_angle }
    }

    pub fn add_random_point(&mut self) {
        let rand = App::gen_random_point();
        self.prev_positions.items.push(rand);
    }

}
