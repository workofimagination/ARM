mod utils;
mod driver;
mod calc;
mod App;
mod stepper;
use calc::{Calc, Point};

fn main() {
    let mut main = App::App::make();

    main.start();

}
