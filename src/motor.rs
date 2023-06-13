use crate::stepper::TestStepper as Stepper;

pub struct Motor {
    motor: Stepper,
    step_degree: f32
}

impl Motor {
    pub fn new(step_degree: f32, dir: u8, step: u8) -> Motor {
        let motor = Stepper::new(dir, step);

        return Motor { motor, step_degree }
    }

    pub fn move_degree(&mut self, degrees: f32) {
        let dir = if i32::signum(degrees as i32) == -1 { false } else { true };
        let steps = (degrees / self.step_degree) as i32;
    } 

    pub fn move_steps(&mut self, steps: i32, dir: bool) {
        for _ in 0..i32::abs(steps) {
            self.motor.step(dir);
        } 
    }
}
