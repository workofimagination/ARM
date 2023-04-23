use tui::widgets::ListState;

pub struct ShiftingVec<T> {
    state: ListState,
    items: Vec<T>
}

impl<T> ShiftingVec<T> {
    pub fn initalize(size: usize) -> ShiftingVec<T> {
        let mut items: Vec<T> = Vec::with_capacity(size);

        unsafe {
            items.set_len(size);
        }

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
