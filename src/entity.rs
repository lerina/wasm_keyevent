pub type Position = (f64, f64);

pub struct Player {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            width: 40.0,
            height: 35.0,
        }
    }

    pub fn get_pos(&self) -> Position {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, pos: Position) {
        (self.x, self.y) = pos;
    }
}
