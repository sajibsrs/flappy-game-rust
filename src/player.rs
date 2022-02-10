use crate::prelude::*;

/// Player character of the game.
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
    
    /// Renders the player on the screen.
    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(20, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    /// Moves player down with time and other screen objects moves left from right.
    pub fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    /// Moves player upward.
    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
