use crate::app::App;

pub struct Handler {
    pub app: App
}

impl Handler {
    pub fn init() -> Handler {
        let app = App::init();

        return Handler { app }
    }

    pub fn start(&mut self) {
        self.app.start(); 
    }
}
