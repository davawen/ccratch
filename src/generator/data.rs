use std::io::{self, Write};
use crate::parser::Data;
use super::{compute_value, Linearize, util::*};

impl Linearize for Data {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Data::SetVariableTo { value, var } => {
                let value = compute_value(f, args, value)?;
                writeln!(f, "{} = {};", get_var(args, &var.id), value)?;
            }
        }
        Ok(Return::Empty)
    }
}
