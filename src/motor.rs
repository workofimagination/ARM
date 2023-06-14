use std::thread;
use std::time::Duration;
use crate::stepper::TestStepper as Stepper;
use crate::calc::Calc;

pub struct Motor {
    motor: Stepper,
    pub step_degree: f32
}

impl Motor {
    pub fn new(step_degree: f32, dir: u8, step: u8) -> Motor {
        let motor = Stepper::new(dir, step);

        return Motor { motor, step_degree }
    }

    pub fn move_degree(&mut self, degrees: f32, delay: i64) {
        let degress_snapped = Calc::snap(degrees, self.step_degree);
        let steps = (degress_snapped / self.step_degree) as i32;

        self.move_steps(steps, delay);
    } 

    pub fn move_smooth(&mut self, delays: Vec<i64>, dir: bool) {
        for delay in delays {
            self.motor.step(!dir);
            thread::sleep(Duration::from_micros(delay as u64));
            self.motor.reset();
            thread::sleep(Duration::from_micros(10));
        }
    }

    pub fn move_steps(&mut self, steps: i32, delay: i64) {
        let dir = if i32::signum(steps) == -1 { false } else { true };

        for _ in 0..i32::abs(steps) {
            self.motor.step(!dir);
            thread::sleep(Duration::from_micros(delay as u64));
            self.motor.reset();
            thread::sleep(Duration::from_micros(10));
        } 
    }
}
