use crate::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

/// Game state
pub struct State {
    player: Player,
    mode: GameMode,
    frame_time: f32,
}

impl State {
    /// Create new instance of State
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
        }
    }

    /// Play the game
    pub fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLUE4);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print_centered(1, "Press SPACE to flap.");

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    /// Restart the game
    pub fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Play;
    }

    /// Action when player is dead
    pub fn dead(&mut self, ctx: &mut BTerm) {
        self.main_menu(ctx);
    }

    /// Renders main menu and actions
    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to flappy");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    /// Gets called in every frame inside game loop
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Play => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
