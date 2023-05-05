use crate::stepper::{self, Stepper};
use std::time::Duration;

pub struct Driver {
    column_joint: Stepper,
    beam_joint: Stepper,
    current_beam_angle: f32,
    current_column_angle: f32,
    step_degree: f32,
    micro_delay: Duration
}

impl Driver {
    pub fn new() -> Driver {
        let column_joint = Stepper::new(0, 0);
        let beam_joint = Stepper::new(0, 0);
        let current_beam_angle = 0.0;
        let current_column_angle = 0.0;
        let step_degree = 1.0/4.0;
        let micro_delay = Duration::from_millis(100);

        return Driver { column_joint, beam_joint, current_beam_angle, current_column_angle, step_degree, micro_delay }
    }
}
