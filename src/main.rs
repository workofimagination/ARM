mod calc;
mod app;
mod handler;
use calc::Calc;
use app::App;

fn main() {
    let mut main = App::make();

    main.start();
}
