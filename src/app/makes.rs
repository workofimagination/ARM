use crate::app::App;

use tui::layout::{Direction, Constraint, Layout};
use tui::symbols;
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Block, Borders, ListItem, List, Dataset, Chart, Axis, GraphType, ListState};

impl App {
    pub fn make_plain_paragraph(&self, content: String) -> Paragraph {
        let paragraph = Paragraph::new(content)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
            );

        return paragraph
    }

    pub fn make_map<'a>(&'a mut self, data: &'a Vec<(f64, f64)>, title: String, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> Chart {
        let dataset = vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .data(data)
        ];
        
        let map = Chart::new(dataset)
            .block(
                Block::default()
                .title(title)
                .borders(Borders::ALL)
            )
            .x_axis(
                Axis::default()
                .bounds(x_bounds)
            )
            .y_axis(
                Axis::default()
                .bounds(y_bounds)
            );

        return map;
    }

    pub fn make_buffer(&mut self) -> Paragraph {
        let buffer = Paragraph::new(self.buffer.clone())
            .style(Style::default())
            .alignment(tui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .border_type(tui::widgets::BorderType::Plain)
            );

        return buffer
    }

    pub fn make_config_window(&mut self) -> Paragraph {
        let config_data = self.make_config_text();
        let config = Paragraph::new(config_data)
            .block(
                Block::default()
                    .title("Info")
                    .borders(Borders::ALL)
            );

        return config
    }

    pub fn make_previous_points(&mut self) -> (List, &mut ListState) {
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

        return (prev_items, self.prev_positions.get_state())
    }

    pub fn make_command_output(&mut self) -> (List, &mut ListState) {
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


        return (command_items, self.prev_positions.get_state())
    }

    pub fn make_current_mode_box(&mut self) -> Paragraph {
        let current_mode_box = Paragraph::new(self.get_current_mode_string())
            .style(Style::default())
            .alignment(tui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .border_type(tui::widgets::BorderType::Plain)
            );

        return current_mode_box
    }

    pub fn make_info_span(left_side: String, right_side: String) -> Spans<'static> {
        let text = Spans::from(vec![
            Span::raw(left_side),
            Span::raw(right_side)
        ]);

        return text
    }

    pub fn make_config_text(&self) -> Vec<Spans>{
        let beam = self.driver.get_beam_position();
        let column = self.driver.get_column_position();

        let text = vec![
            App::make_info_span(String::from("DELAY: "), format!("{}", self.driver.micro_delay_default)),
            App::make_info_span(String::from("MAX DELAY: "), format!("{}", self.driver.micro_delay_max)),
            App::make_info_span(String::from("MIN DELAY: "), format!("{}", self.driver.micro_delay_min)),
            App::make_info_span(String::from("MVNT AMT: "), format!("{}", self.driver.movement_amount)),
            App::make_info_span(String::from("BEAM X: "), format!("{}", beam.x)),
            App::make_info_span(String::from("BEAM Y: "), format!("{}", beam.y)),
            App::make_info_span(String::from("COLUMN X: "), format!("{}", column.x)),
            App::make_info_span(String::from("COLUMN Y: "), format!("{}", column.y)),
            App::make_info_span(String::from("STEP AMT: "), format!("{}", self.driver.generic_step_amount)),
        ];

        return text
    }

    pub fn make_chunk(direction: Direction, constraints: Vec<Constraint>) -> Layout {
        let chunk = Layout::default()
            .direction(direction)
            .margin(0)
            .constraints(constraints.as_ref());

        return chunk
    }

}
