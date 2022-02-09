mod state;
mod mode;

mod prelude {
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;
}

use crate::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy")
        .build()?;

    main_loop(context, State::new())
}
