use crate::prelude::*;

/// Game obstacle
pub struct Obstacle {
    pub x: i32,
    gap_center_y: i32,
    gap_size: i32,
}

impl Obstacle {
    /// New instance of Obstacle
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle { 
            x,
            gap_center_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }

    /// Render obstacle
    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.gap_size / 2;

        // Obstacle top portion
        for y in 0..self.gap_center_y - half_size {
            ctx.set(
                screen_x,
                y,
                GREEN,
                BLACK,
                to_cp437('#')
            );
        }

        // Obstacle bottom portion
        for y in self.gap_center_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                GREEN,
                BLACK,
                to_cp437('#')
            );
        }
    }

    /// Collition detection for obstacle and player
    pub fn hit(&self, player: &Player) -> bool {
        let half_size = self.gap_size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_center_y - half_size;
        let player_below_gap = player.y > self.gap_center_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
