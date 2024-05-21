use std::io::{self, Write};
use crate::parser::Event;
use super::{compute_value, Linearize, util::*};

impl Linearize for Event {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Event::WhenFlagClicked => {
                writeln!(f, "if (g->flag_clicked) s->state = {};", *args.state + 1)?;
                Ok(Return::Hold)
            }
        }
    }
}
