use crate::App::AngleSet;

use std::fs::File;
use std::io::Write;
use std::clone::Clone;

use tui::widgets::ListState;

pub struct ShiftingVec<T> where T: Clone {
    state: ListState,
    items: Vec<T>,
    default_value: T,
    size: usize
}

impl<T> ShiftingVec<T> where T: Clone {
    pub fn initalize(size: usize, default_value: T) -> ShiftingVec<T> {
        let mut items: Vec<T> = Vec::with_capacity(size);

        //this is actually safe don't worry about it promise :)
        //correction - this causes weird behavour

        unsafe { items.set_len(size); }

        let state = ListState::default();

        let shifting_vec = ShiftingVec { state, items, default_value,  size };
    
        return shifting_vec;
    }

    pub fn insert(&mut self, item: T) {
        self.items.rotate_left(1);
        let length = &self.items.len();
        self.items[length-1] = item;
    }

    pub fn set_size(&mut self, size: usize) {
        if size > self.size {
            for _ in 0..size - self.size {
                self.items.push(self.default_value.clone());
            }

            self.items.rotate_right(size - self.size);
        } else {
            for _ in 0..self.size - size {
                self.items.remove(0);
            }
        }

        self.size = size;
    }

    pub fn flush(&mut self) {
        for _ in 0..self.items.len() {
            self.insert(self.default_value.clone());
        }
    }

    pub fn set_all(&mut self, item: T) {
        for _ in 0..self.items.len() {
            self.insert(item.clone());
        }
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

//I stole this shit from stackoverflow
pub mod Util_Macros {
    macro_rules! tuple_into {
        ($t: expr, ($($ty: ident), *)) => {
            {
                let ($($ty,)*) = $t;
                ($($ty as $ty,) *)
            }
        }
    }

    pub(crate) use tuple_into;
}

