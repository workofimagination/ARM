mod utils;
mod calc;
mod driver;
mod app;
mod stepper;

use calc::Calc;
use app::App;

fn main() {
    let mut main = App::make();

    main.start();
}
