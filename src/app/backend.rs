use crate::app::{App, AngleSet, Mode};
use crate::driver::DriverError;

use crate::driver;
use crate::utils::{Utils, ShiftingVec};

use std::num::ParseFloatError;

use rand::Rng;


impl App {
    pub fn gen_random_point() -> AngleSet {
        let mut rng = rand::thread_rng();
        let column_angle = rng.gen_range(0.1..2.0);
        let beam_angle = rng.gen_range(-1.0..2.0);

        return AngleSet { column_angle, beam_angle, rotation_angle: 0.0 }
    }

    pub fn add_random_point(&mut self) {
        //these are between 1 and 2, not actual angles I have no idea why these are still here
        //but im keeping them in for testing purposes

        let rand = App::gen_random_point();
        self.buffer = format!("{} {}", rand.column_angle, rand.beam_angle);
        self.goto();
    }

    pub fn add_current_position(&mut self) {
        let beam_angle = self.driver.get_beam_angle();
        let column_angle = self.driver.get_column_angle();

        let current_position = AngleSet {beam_angle, column_angle, rotation_angle: 0.0 };

        self.prev_positions.insert(current_position);
    }

    pub fn get_current_position(&mut self) -> AngleSet {
        let beam_angle = self.driver.get_beam_angle();
        let column_angle = self.driver.get_column_angle();

        return AngleSet {beam_angle, column_angle, rotation_angle: 0.0 };
    }

    pub fn move_direction(&mut self, dir: driver::Direction) {
        match self.driver.move_direction(dir) {
            Ok(()) => (), 
            Err(e) => { self.handle_driver_error_generic(e) }
        }
    }

    pub fn goto_smooth(&mut self) {
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
            Err(e) => { self.handle_driver_error_generic(e) }
        }
    }

    pub fn goto(&mut self) {
        let (x, y) = match self.parse_buffer_goto() {
            Ok(x) => x,
            Err(e) => {
                self.command_output.insert(format!("{}", e));
                return
            }
        };

        self.command_output.insert(format!("successfully parsed buffer"));

        let current_poistion = self.get_current_position();

        match self.driver.goto_point(x, y) {
            Ok(()) => {
                self.command_output.insert(format!("successfully wennt to point {} {}", x, y));
                self.prev_positions.insert(current_poistion);
            },

            Err(e) => { self.handle_driver_error_generic(e) }
        }
    }

    pub fn parse_buffer_goto(&self) -> Result<(f32, f32), ParseFloatError> {
        let coords = self.buffer.split(" ").collect::<Vec<&str>>();

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

    pub fn get_current_mode_string(&self) -> &str {
        let string = match self.current_mode {
            Mode::Normal => { "Normal" },
            Mode::Control => { "Control" },
            Mode::Buffer => { "Buffer" }
        };

        return string
    } 

    pub fn get_current_x(&self) -> f32 {
        return self.driver.current_position.x
    }

    pub fn get_current_y(&self) -> f32 {
        return self.driver.current_position.y
    }

    pub fn get_current_z(&self) -> f32 {
        return self.driver.current_position.z
    }

    pub fn get_current_column_angle(&self) -> f32 {
        return self.driver.column_angle;
    }

    pub fn get_current_beam_angle(&self) -> f32 {
        return self.driver.beam_angle;
    }

    pub fn get_current_base_angle(&self) -> f32 {
        return self.driver.base_angle;
    }

    pub fn save_current_angles(&mut self) {
        let current_angles = self.get_current_angle_set();

        let save_string = format!("{} {} {}", current_angles.column_angle, current_angles.beam_angle, current_angles.rotation_angle);

        match Utils::save_to_file("./output".to_string(), save_string) {
            Ok(x) => self.command_output.insert(x),
            Err(e) => self.command_output.insert(format!("unable to save to file: {}", e))
        }
    }

    pub fn get_current_angle_set(&self) ->  AngleSet {
        let beam_angle = self.get_current_beam_angle();
        let column_angle = self.get_current_column_angle();

        return AngleSet { column_angle, beam_angle, rotation_angle: 0.0 }
    }

    pub fn get_2d_points(&self) -> Vec<(f64, f64)>{
        let column = self.driver.get_column_position();

        let beam = self.driver.get_beam_position();

        return vec![(0.0,0.0), (column.x as f64, column.y as f64), (beam.x as f64, beam.y as f64)]
    }

    //this function may not need to exist
    pub fn get_x_z_points(&self) -> Vec<(f64, f64)> {
        let return_vec = vec![(0.0, 0.0), (self.driver.current_position.x as f64, self.driver.current_position.z as f64)];

        return return_vec
    }

    pub fn increase_movement_amount(&mut self) {
        self.driver.movement_amount *= 1.25;
    }

    pub fn decrease_movement_amount(&mut self) {
        self.driver.movement_amount /= 1.25;
    }

    pub fn increase_max_delay(&mut self) {
        self.driver.micro_delay_max += 10;
    }

    pub fn decrease_max_delay(&mut self) {
        if self.driver.micro_delay_max <= 10 || self.driver.micro_delay_max <= self.driver.micro_delay_min + 10 {
            return
        }

        self.driver.micro_delay_max -= 10;
    }

    pub fn increase_min_delay(&mut self) {
        if self.driver.micro_delay_min >= self.driver.micro_delay_max - 10 {
            return
        }

        self.driver.micro_delay_min += 10;
    }

    pub fn decrease_min_delay(&mut self) {
        if self.driver.micro_delay_min <= 10 {
            return
        }

        self.driver.micro_delay_min -= 10;
    }

    pub fn increase_delay(&mut self) {
        self.driver.micro_delay_default += 10;
    }

    pub fn decrease_delay(&mut self) {
        if self.driver.micro_delay_default <= 10 {
            return
        }

        self.driver.micro_delay_default -= 10;
    }

    pub fn flush_prev_positions(&mut self) {
        self.prev_positions.flush();
    }

    pub fn flush_command_output(&mut self) {
        self.command_output.flush();
    }

    pub fn set_shifting_vec_size<T>(size: usize, vec: &mut ShiftingVec<T>) where T: Clone {
        vec.set_size(size);
    }

    pub fn increase_prev_points(&mut self) {
        self.prev_positions_size += 1;

        App::set_shifting_vec_size(self.prev_positions_size, &mut self.prev_positions);
    }

    pub fn decrease_prev_points(&mut self) {
        if self.prev_positions_size == 1 {
            return 
        }

        self.prev_positions_size -= 1;

        App::set_shifting_vec_size(self.prev_positions_size, &mut self.prev_positions)
    }

    pub fn increase_command_ouput(&mut self) {
        self.command_output_size += 1;

        App::set_shifting_vec_size(self.command_output_size, &mut self.command_output);
    }

    pub fn decrease_command_output(&mut self) {
        if self.command_output_size == 1 {
            return 
        }

        self.command_output_size -= 1;

        App::set_shifting_vec_size(self.command_output_size, &mut self.command_output)
    }

    pub fn handle_driver_error_generic(&mut self, error: DriverError) {
        match error {
            DriverError::UnReachable => {
                let error_message = String::from("unable to reach target position, out of range");
                self.command_output.insert(error_message);
            },
            
            DriverError::CantNormalize => {
                let error_message = String::from("unable to normalize derived smooth, most likely a divide by zero issue");
                self.command_output.insert(error_message);
            }
        }
    }
}

