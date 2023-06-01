use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, JoinHandle};
use rand::Rng;
use crate::stepper::TestStepper;
use crate::calc::Calc;
use crate::utils::{ Point, AngleSet };

pub struct Driver {
    pub column_motor: Arc<Mutex<TestStepper>>,
    pub beam_motor: Arc<Mutex<TestStepper>>,
    pub base_motor: Arc<Mutex<TestStepper>>,
    pub column_angle: f32,
    pub beam_angle: f32,
    pub base_angle: f32,
    pub step_degree: f32,
    pub movement_amount: f32,
    pub micro_delay_default: i64,
    pub micro_delay_max: i64,
    pub micro_delay_min: i64,
    pub current_position: Point,
    pub calc: Calc
}

#[derive(Debug)]
pub enum DriverError {
    UnReachable,
    CantNormalize
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
} 

impl Driver {
    pub fn new() -> Driver {
        let column_motor = Arc::new(Mutex::new(TestStepper::new(0, 0)));
        let beam_motor = Arc::new(Mutex::new(TestStepper::new(0, 0)));
        let base_motor = Arc::new(Mutex::new(TestStepper::new(0, 0)));
        let column_angle = 0.0;
        let beam_angle = 0.0;
        let base_angle = 0.0;
        let step_degree = 1.0/4.0;
        let movement_amount = 0.01;
        let micro_delay_default = 0;
        let micro_delay_min = 1750;
        let micro_delay_max = 4000;
        let current_position = Point { x: 2.0, y: 0.0, z: 0.0 };
        let calc = Calc::new(0.0, 0.0, 1.0);

        return Driver { column_motor, beam_motor, base_motor, column_angle, beam_angle, base_angle, step_degree,
                        movement_amount,micro_delay_default, micro_delay_max, micro_delay_min, current_position, calc 
        }
    }

    pub fn get_random_angle() -> f32 {
        let mut rng = rand::thread_rng();

        let angle = rng.gen_range(-180.0..180.0);

        return angle;
    }

    pub fn goto_point(&mut self, x: f32, y: f32) -> Result<(), DriverError>{
        let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

        if Calc::dist(self.calc.origin.x, self.calc.origin.y, x, y) > self.calc.radius*2.0 { return Err(DriverError::UnReachable) }

        let angles = self.calc.get_angles(x, y);

        let (beam_steps, column_steps, column_snapped, beam_snapped) = self.get_steps_2d(angles.column_angle, angles.beam_angle);

        let column_dir = if i32::signum(beam_steps) == -1 { false } else { true };
        let beam_dir = if i32::signum(column_steps) == -1 { false } else { true };

        let column_thread = Driver::move_motor(&mut self.column_motor, column_steps, column_dir, self.micro_delay_default);
        thread_pool.push(column_thread);
            
        let beam_thread = Driver::move_motor(&mut self.beam_motor, beam_steps, beam_dir, self.micro_delay_default);
        thread_pool.push(beam_thread);

        for thread in thread_pool {
            match thread.join() {
                Ok(()) => (),
                Err(e) => { println!("{:?}", e); }
            }
        }

        self.column_angle = column_snapped;
        self.beam_angle = beam_snapped;

        let cur_pos = self.get_current_position();

        self.current_position.x = cur_pos.x;
        self.current_position.y = cur_pos.y;

        Ok(())
    }

    pub fn goto_point_smooth(&mut self, x: f32, y: f32) -> Result<(), DriverError>{
        let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

        if Calc::dist(self.calc.origin.x, self.calc.origin.y, x, y) > self.calc.radius*2.0 { return Err(DriverError::UnReachable) }

        let angles = self.calc.get_angles(x, y);

        let (beam_steps, column_steps, column_snapped, beam_snapped) = self.get_steps_2d(angles.column_angle, angles.beam_angle); 

        let column_dir = if i32::signum(beam_steps) == -1 { false } else { true };
        let beam_dir = if i32::signum(column_steps) == -1 { false } else { true };

        let column_liner = Driver::get_linear_steps(column_steps);
        let beam_linear = Driver::get_linear_steps(beam_steps);

        let column_smoothed = Calc::smooth(column_liner);
        let beam_smoothed = Calc::smooth(beam_linear);

        match Calc::normalize_vec(self.micro_delay_min, self.micro_delay_max, column_smoothed) {
            Some(x) => {
                let thread = Driver::move_motor_smooth(&mut self.column_motor, x, column_dir);
                thread_pool.push(thread);
            },

            None => { return Err(DriverError::CantNormalize) }
        };

        match Calc::normalize_vec(self.micro_delay_min, self.micro_delay_max, beam_smoothed) {
            Some(x) => {
                let thread = Driver::move_motor_smooth(&mut self.beam_motor, x, beam_dir);
                thread_pool.push(thread);
            },

            None => { return Err(DriverError::CantNormalize) }
        };

        for thread in thread_pool {
            thread.join().unwrap();
        }

        self.column_angle = column_snapped;
        self.beam_angle = beam_snapped;

        let cur_pos = self.get_current_position();

        self.current_position.x = cur_pos.x;
        self.current_position.y = cur_pos.y;

        Ok(())
    }

    pub fn goto_point_3d(&mut self, x: f32, y: f32, z: f32) -> Result<(), DriverError> {
        if Calc::dist_3d(&self.current_position, &Point { x, y, z }) > self.calc.radius*2.0 { return Err(DriverError::UnReachable) }

        let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

        let angles = self.calc.get_angles_3d(x, y ,z);
        let (column_steps, beam_steps, base_steps,
                column_snapped, beam_snapped, base_snapped) = self.get_steps_3d(angles.column_angle, 
                                                                                angles.beam_angle, angles.base_angle);

        let column_dir = if i32::signum(beam_steps) == -1 { false } else { true };
        let beam_dir = if i32::signum(column_steps) == -1 { false } else { true };
        let base_dir = if i32::signum(base_steps) == -1 { false } else { true };

        let column_thread = Driver::move_motor(&mut self.column_motor, column_steps, column_dir, self.micro_delay_default);
        thread_pool.push(column_thread);
            
        let beam_thread = Driver::move_motor(&mut self.beam_motor, beam_steps, beam_dir, self.micro_delay_default);
        thread_pool.push(beam_thread);

        let base_thread = Driver::move_motor(&mut self.base_motor, base_steps, base_dir, self.micro_delay_default);
        thread_pool.push(base_thread);

        for thread in thread_pool {
            match thread.join() {
                Ok(()) => (),
                Err(e) => { println!("{:?}", e); }
            }
        }

        self.column_angle = column_snapped;
        self.beam_angle = beam_snapped;
        self.base_angle = base_snapped;

        self.current_position.x = x;
        self.current_position.y = y;
        self.current_position.z = z;
        Ok(())
    }

    pub fn move_direction(&mut self, direction: Direction) -> Result<(), DriverError> {
        match direction {
            Direction::Left => {
                return self.goto_point(self.current_position.x - self.movement_amount, self.current_position.y);                
            },

            Direction::Right => {
                return self.goto_point(self.current_position.x + self.movement_amount, self.current_position.y);
            },

            Direction::Up => {
                return self.goto_point(self.current_position.x, self.current_position.y + self.movement_amount);
            },
            
            Direction::Down => {
                return self.goto_point(self.current_position.x, self.current_position.y - self.movement_amount);
            }
        } 
    }

    pub fn move_motor(motor: &mut Arc<Mutex<TestStepper>>, steps: i32, dir: bool, delay: i64) -> JoinHandle<()> {
        let motor = Arc::clone(motor);

        thread::spawn( move || {
            let mut motor = motor.lock().unwrap();
            let delay = Duration::from_micros(delay as u64);

            for _ in 0..i32::abs(steps) {
                motor.step(dir);
                thread::sleep(delay);
                motor.reset();
                thread::sleep(delay);
            }
        })
    }

    pub fn move_motor_smooth(motor: &mut Arc<Mutex<TestStepper>>, delays: Vec<i64>, dir: bool) -> JoinHandle<()> {
        let motor = Arc::clone(motor);

        thread::spawn(move || {
            let mut motor = motor.lock().unwrap();

            for delay in delays {
                motor.step(dir);
                thread::sleep(Duration::from_micros(delay as u64));
                motor.reset();
                thread::sleep(Duration::from_micros(delay as u64));
            }
        })
    }

    pub fn get_steps_2d(&self, column_angle: f32, beam_angle: f32) -> (i32, i32, f32, f32) { //this
        //return type is retarded. too bad
        let column_snapped = Calc::snap(Calc::to_degree(column_angle), self.step_degree);
        let beam_snapped = Calc::snap(Calc::to_degree(beam_angle), self.step_degree);

        let change_in_column = column_snapped - self.column_angle;
        let change_in_beam = beam_snapped - self.beam_angle - self.column_angle;

        let column_steps = (change_in_column/self.step_degree) as i32;
        let beam_steps = (change_in_beam/self.step_degree) as i32;

        return (column_steps, beam_steps, column_snapped, beam_snapped)
    }

    //this is also retarded
    pub fn get_steps_3d(&self, column_angle: f32, beam_angle: f32, base_angle: f32) -> (i32, i32, i32, f32, f32, f32) {
        let (column_steps, beam_steps, column_snapped, beam_snapped) = self.get_steps_2d(column_angle, beam_angle);

        let base_snapped = Calc::snap(Calc::to_degree(base_angle), self.step_degree);
        
        let change_in_base = base_snapped - self.base_angle; 

        let base_steps = (change_in_base/self.step_degree) as i32;

        return (
                column_steps, beam_steps, base_steps,
                column_snapped, beam_snapped, change_in_base
        )
    }

    pub fn get_column_position(&self) -> Point {
        let column = Calc::get_point(Calc::to_radian(self.column_angle), &self.calc.origin);

        return Point { x: column.x, y: column.y, z: 0.0 }
    }

    pub fn get_beam_position(&self) -> Point {
        let column = Calc::get_point(Calc::to_radian(self.column_angle), &self.calc.origin);

        let beam = Calc::get_point(Calc::to_radian(self.beam_angle), &column);

        return Point { x: beam.x, y: beam.y, z: 0.0 }
    } 

    pub fn get_beam_angle(&self) -> f32 {
        return self.beam_angle  
    }

    pub fn get_column_angle(&self) -> f32 {
        return self.column_angle 
    }

    fn get_linear_steps(steps: i32) -> Vec<i64> {
        let mut counter = 1;
        let mut times: Vec<i64> = Vec::new();

        (0..i32::abs(steps)).for_each(|_| { times.push(counter); counter += 100 });

        return times;
    }

    pub fn get_current_position(&self) -> Point {
        let pos = self.get_column_position();
        let beam_angle = self.get_beam_angle();
        let center = Point { x: pos.x as f32, y: pos.y as f32, z: 0.0 };

        let current_position = Calc::get_point(Calc::to_radian(beam_angle), &center);

        return current_position;
    }
}
