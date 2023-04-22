mod calc;
mod app;
mod handler;
use calc::Calc;
use app::App;
use handler::Handler;

fn main() {
    let mut handler = Handler::init();

    handler.start();
}
