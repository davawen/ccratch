use std::io::{self, Write};
use crate::parser::Motion;
use super::{compute_value, Linearize, util::*};

impl Linearize for Motion {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Motion::MoveSteps { steps } => {
                let steps = compute_value(f, args, steps)?;
                writeln!(f, "convert_to_number(&{steps});")?;
                writeln!(f, "float direction = scratch_degrees_to_radians(a->actor_state.direction);")?;
                writeln!(f, "a->actor_state.x += cosf(direction)*{steps}.n;")?;
                writeln!(f, "a->actor_state.y += sinf(direction)*{steps}.n;")?;
            }
        }

        Ok(Return::Empty)
    }
}
