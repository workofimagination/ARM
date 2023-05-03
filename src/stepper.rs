use rppal::gpio::{OutputPin, Gpio};

pub struct Stepper {
    direction_pin: u8,
    step_pin: u8,
    dir: OutputPin,
    step: OutputPin,
}

impl Stepper {
    pub fn new(direction_pin: u8, step_pin: u8) -> Self {
        let dir = Gpio::new().unwrap().get(direction_pin).unwrap().into_output();
        let step = Gpio::new().unwrap().get(step_pin).unwrap().into_output();
        Self { direction_pin, step_pin, dir, step }
    }

    pub fn step(&mut self, direction: bool) {
        if direction { self.dir.set_high() }
        else { self.dir.set_low() }
        self.step.set_high();
    }

    pub fn reset(&mut self) { // must call reset before calling step() again
        self.step.set_low();
    }

    // pub fn step(&mut self, steps: usize, direction: bool) { // direction == true CW || direction == false CCW
    //     if (direction) { self.dir.set_high() }
    //     else { self.dir.set_low() }
    //     for x in 1..=steps*self.divisions {
    //         self.step.set_high();
    //         thread::sleep(self.delay);
    //         self.step.set_low();
    //     }
    // }

}
