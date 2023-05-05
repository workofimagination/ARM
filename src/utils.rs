use std::fs::File;
use std::io::Write;

use tui::widgets::ListState;

pub struct ShiftingVec<T> {
    state: ListState,
    items: Vec<T>
}

impl<T> ShiftingVec<T> {
    pub fn initalize(size: usize) -> ShiftingVec<T> {
        let mut items: Vec<T> = Vec::with_capacity(size);

        //this is actually safe don't worry about it promise :)
        unsafe { items.set_len(size); }

        let state = ListState::default();
    
        return ShiftingVec { state, items }
    }

    pub fn insert(&mut self, item: T) {
        self.items.rotate_left(1);
        let length = &self.items.len();
        self.items[length-1] = item;
    }

    pub fn get_items(&self) -> &Vec<T> {
        return &self.items;
    }

    pub fn get_state(&mut self) -> &mut ListState {
        return &mut self.state;
    }
}

pub struct Utils;

impl Utils {
    pub fn save_to_file(path: String, contents: String) -> Result<String, std::io::Error>{
        let mut file = match File::create(format!("{}", path)) {
            Ok(x) => x,
            Err(e) => return Err(e)
        };

        match file.write(contents.as_bytes()) {
            Ok(_) => return Ok(format!("successfully saved to {}", path)),
            Err(e) => return Err(e)
        }
    }
}

