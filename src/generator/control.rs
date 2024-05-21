use std::io::{self, Write};
use crate::parser::Control;
use super::{compute_value, linearize_sequence, Linearize, util::*};

impl Linearize for Control {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Control::CreateCloneOf { actor: _ } => {
                // TODO:
                writeln!(f, "printf(\"TODO: create clone code\");")?;
                writeln!(f, "exit(-1);")?;
            }
            Control::Wait { duration } => {
                let duration = compute_value(f, args, duration)?;
                writeln!(f, "convert_to_number(&{duration});")?;
                writeln!(f, "s->time = GetTime() + {duration}.n;")?;
                writeln!(f, "s->state = {};", *args.state + 1)?;

                end_case(f, args.state)?;
                start_case(f, args.state)?;

                writeln!(f, "if (GetTime() >= s->time) s->state = {};", *args.state + 1)?;
                return Ok(Return::Hold);
            }
            Control::Repeat { times, branch } => {
                *args.state += 1; // make the branch think we ended this case
                let repeat_start = *args.state;
                let mut branch_code = IW::new(Vec::new());
                // prepare branch to get end state number
                linearize_sequence(&mut branch_code, args, branch)?;

                // initialize loop value (evaluate condition once)
                let num = compute_value(f, args, times)?;
                writeln!(f, "convert_to_number(&{num})")?;

                // if initial condition is false, skip loop body
                writeln!(f, "if ((int){num}.n <= 0) s->state = {};", *args.state + 1)?;

                // otherwise, initialize loop variable and go to the next state (start of the sequence we linearized above)
                writeln!(f, "else {{")?;
                f.indent();
                writeln!(f, "s->state = {};", repeat_start)?;
                let loop_var = format!("loop_{}", generate_var_name());
                writeln!(f, "s->{loop_var} = {num}.n;")?;
                f.deindent();
                writeln!(f, "}}")?;

                *args.state -= 1; // reverse previous add
                                  // end loop start
                end_case(f, args.state)?;

                f.write_all(&branch_code.writer)?;

                // end loop (loop back condition)
                start_case(f, args.state)?;
                writeln!(f, "s->{loop_var}--;")?;
                writeln!(f, "if (s->{loop_var} > 0) s->state = {};", repeat_start)?;
                writeln!(f, "else s->state = {};", *args.state + 1)?;
                args.new_locals.push(loop_var);
                return Ok(Return::Hold);
            }
            Control::IfCondition { condition, branch } => {
                *args.state += 1; // make the branch think we ended this case
                let branch_start = *args.state;
                let mut branch_code = IW::new(Vec::new());
                // prepare branch to get end state number
                linearize_sequence(&mut branch_code, args, branch)?;

                let condition = compute_value(f, args, condition)?;
                writeln!(f, "convert_to_bool(&{condition});")?;
                writeln!(f, "if ({condition}.b) s->state = {branch_start};")?;
                writeln!(f, "else s->state = {};", *args.state + 1)?;

                *args.state -= 1; // reverse previous add
                                  // end condition start
                end_case(f, args.state)?;
                f.write_all(&branch_code.writer)?;

                return Ok(Return::Ended)
            }
        }
        Ok(Return::Empty)
    }
}
