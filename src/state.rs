use crate::{prelude::*, mode::GameMode};

/// Game state
pub struct State {
    mode: GameMode,
}

impl State {
    /// Create new instance of State
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    /// Play the game
    pub fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    /// Restart the game
    pub fn restart(&mut self) {
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
