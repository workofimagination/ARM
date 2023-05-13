mod utils;
mod driver;
mod calc;
mod app;
mod stepper;
use calc::{Calc, Point};
use app::App;

fn main() {
    let mut main = App::make();

    main.start();

}
