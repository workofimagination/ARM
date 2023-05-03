use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::stepper::Stepper;
use crate::calc::Calc;

struct Driver {
    column_motor: Stepper,
    current_column_angle: f32,
    beam_motor: Stepper,
    current_beam_angle: f32,
    step_degree: f32,
    micro_delay: Duration,
    temp: Arc<Mutex<Stepper>>
}

impl Driver {
    pub fn new() -> Driver {
        let column_motor = Stepper::new(0, 0);
        let current_beam_angle = 0.0;
        let beam_motor = Stepper::new(0, 0);
        let current_column_angle = 0.0;
        let step_degree = 3.0/2.0;
        let micro_delay = Duration::from_millis(100);

        let temp = Arc::new(Mutex::new(Stepper::new(0, 0)));

        return Driver { column_motor, current_beam_angle, beam_motor, current_column_angle, step_degree, micro_delay, temp }
    }

    pub fn move_column(&mut self, angle: f32) {
        let snapped = Calc::snap(angle, self.step_degree);
        let change = snapped - self.current_column_angle;

        let steps = (change/self.step_degree) as i32;
        let dir = if f32::signum(change) as i32 == -1 { false } else { true };

        let output = Arc::clone(&self.temp);

        let delay = self.micro_delay.clone();

        thread::spawn(move || {
            let mut t = output.lock().unwrap();
            t.step(dir);
            thread::sleep(delay);
            t.reset();
            thread::sleep(delay);
        });
    }
}
