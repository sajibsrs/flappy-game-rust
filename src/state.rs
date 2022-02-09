use crate::prelude::*;

pub struct State {

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Rust bracket-lib terminal!");
    }
}
