mod utils;
mod driver;
mod calc;
mod app;
mod stepper;

fn main() {
    let mut main = app::App::make();

    main.start();

}
