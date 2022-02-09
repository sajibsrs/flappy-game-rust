use crate::{prelude::*, mode::GameMode};

pub struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    pub fn restart(&mut self) {
        self.mode = GameMode::Play;
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        self.main_menu(ctx);
    }

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
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Play => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
