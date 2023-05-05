use crate::calc::{Calc, Point};
use crate::stepper::{self, Stepper};
use std::sync::{Arc, Mutex};
use std::time::Duration;

enum DriverError {
    OutOfRange(String),
}

pub struct Driver {
    column_joint: Arc<Mutex<Stepper>>,
    beam_joint: Arc<Mutex<Stepper>>,
    current_beam_angle: f32,
    current_column_angle: f32,
    step_degree: f32,
    micro_delay: Duration,
    calculator: Calc
}

impl Driver {
    pub fn new() -> Driver {
        let column_joint = Arc::new(Mutex::new(Stepper::new(0, 0)));
        let beam_joint = Arc::new(Mutex::new(Stepper::new(0, 0)));
        let current_beam_angle = 0.0;
        let current_column_angle = 0.0;
        let step_degree = 1.0/4.0;
        let micro_delay = Duration::from_millis(100);
        let calculator = Calc::new(0.0, 0.0, 1.0);

        return Driver { column_joint, beam_joint, current_beam_angle, current_column_angle, step_degree, micro_delay, calculator }
    }

    pub fn goto(&mut self, x: f32, y: f32) -> Result<(), DriverError> {
        let goto_point = Point{ x, y };
        let (goto_column, goto_beam) = self.calculator.get_angles(&goto_point);

        if goto_column.is_nan() || goto_beam.is_nan() {
            let error = DriverError::OutOfRange(String::from("ARM IS OUT OF RANGE"));
            return Err(error)
        }
        
        let change_in_column = goto_column - self.current_column_angle;
        let change_in_beam = goto_beam - self.current_beam_angle - self.current_column_angle;

        //move joints

        self.current_beam_angle = goto_beam;
        self.current_column_angle = goto_column;

        Ok(())
    }

    pub fn move_motor(&mut self, motor: &mut Arc<Mutex<Stepper>>, change: f32) -> Result<(), DriverError> {
        let steps = (change/self.step_degree) as i32; 
        let dir = if f32::signum(change) as i32 == -1 { false } else { true };
        let delay = self.micro_delay.clone();

        let output = Arc::clone(motor);

        Ok(())
    }

    pub fn manual_zero(&mut self) {
        self.current_beam_angle = 0.0;
        self.current_column_angle = 0.0;
    }
}
