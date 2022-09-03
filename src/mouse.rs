#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    last_event: u8,
}

impl MouseState {
    pub fn new() -> Self {
        println!("MouseState::new");
        Self { last_event: 0 }
    }

    pub fn down(&mut self) {
        println!("MouseState::down");
        self.last_event = 1
    }

    pub fn down_middle(&mut self) {
        println!("MouseState::down_middle");
        self.last_event = 4
    }

    pub fn down_right(&mut self) {
        println!("MouseState::down_right");
        self.last_event = 6
    }

    pub fn moving(&mut self) {
        // println!("MouseState::moving");
        match self.last_event {
            1 => self.last_event = 2,
            2 => self.last_event = 2,
            _ => self.last_event = 0,
        }
    }

    pub fn release(&mut self) {
        println!("MouseState::release");
        match self.last_event {
            2 => self.last_event = 3,
            _ => self.last_event = 0,
        }
    }

    pub fn release_middle(&mut self) {
        println!("MouseState::release_middle");
        match self.last_event {
            4 => self.last_event = 5,
            _ => self.last_event = 0,
        }
    }

    pub fn release_right(&mut self) {
        println!("MouseState::release_right");
        match self.last_event {
            6 => self.last_event = 7,
            _ => self.last_event = 0,
        }
    }

    pub fn is_select(&mut self) -> bool {
        if self.last_event == 3 {
            self.last_event = 0;
            true
        } else {
            false
        }
    }

    pub fn middle_clicked(&mut self) -> bool {
        if self.last_event == 5 {
            self.last_event = 0;
            true
        } else {
            false
        }
    }

    pub fn right_clicked(&mut self) -> bool {
        if self.last_event == 7 {
            self.last_event = 0;
            true
        } else {
            false
        }
    }
}
