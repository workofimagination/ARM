use std::time::Duration;

use crate::stepper::Stepper;
struct Driver {
    column_motor: Stepper,
    current_column_angle: f32,
    beam_motor: Stepper,
    current_beam_angle: f32,
    step_degree: f32,
    micro_delay: Duration
}

impl Driver {
    pub fn new() -> Driver {
        let column_motor = Stepper::new(0, 0);
        let current_beam_angle = 0.0;
        let beam_motor = Stepper::new(0, 0);
        let current_column_angle = 0.0;
        let step_degree = 3.0/2.0;
        let micro_delay = Duration::from_millis(100);

        return Driver { column_motor, current_beam_angle, beam_motor, current_column_angle, step_degree, micro_delay }
    }
}
