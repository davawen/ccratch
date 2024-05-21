use std::io::{self, Write};
use crate::parser::Looks;
use super::{compute_value, Linearize, util::*};

impl Linearize for Looks {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Looks::SayForSecs { message, secs } => {
                // printing part
                let message = compute_value(f, args, &message)?;
                writeln!(f, "Value output = copy_value({message});")?;
                writeln!(f, "convert_to_rcstr(&output);")?;

                let duration = compute_value(f, args, secs)?;
                writeln!(f, "convert_to_number(&{duration});")?;
                writeln!(f, "s->time = GetTime() + {duration}.n;")?;
                writeln!(f, "s->state = {};", *args.state + 1)?;

                writeln!(f, "a->actor_state.saying = output.s;")?;
                writeln!(f, "a->actor_state.say_end = s->time;")?;

                // waiting part
                end_case(f, args.state)?;
                start_case(f, args.state)?;

                writeln!(f, "if (GetTime() >= s->time) s->state = {};", *args.state + 1)?;
                return Ok(Return::Hold);
            }
        }
    }
}
