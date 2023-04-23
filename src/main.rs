mod utils;
mod calc;
mod app;
use calc::Calc;
use app::App;

fn main() {
    let mut main = App::make();

    main.start();
}
