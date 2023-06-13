mod utils;
mod driver;
mod calc;
mod app;
mod stepper;
mod motor;

fn main() {
    let mut main = app::App::new();

    main.start();
}
