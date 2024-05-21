use std::io::{self, Write};
use crate::parser::{Value, Operator};
use super::{compute_value, Linearize, util::*};

/// `op_func` is the function that will get applied to both operands.
///
/// Returns a variable's identifier.
fn binop_block<W: Write>(
    f: &mut IW<W>,
    args: &mut GeneratorArgs,
    lhs: &Value,
    rhs: &Value,
    op_func: &str,
) -> io::Result<Return> {
    let lhs = compute_value(f, args, lhs)?;
    let rhs = compute_value(f, args, rhs)?;

    writeln!(f, "{lhs} = {op_func}({lhs}, {rhs});")?;
    Ok(Return::Value(lhs))
}

impl Linearize for Operator {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return> {
        match self {
            Operator::Add { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_add")?),
            Operator::Sub { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_sub")?),
            Operator::Mul { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_mul")?),
            Operator::Div { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_div")?),
            Operator::GreaterThan { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_greater_than")?),
            Operator::LesserThan { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_lesser_than")?),
            Operator::Equals { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_equal")?),
            Operator::And { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_and")?),
            Operator::Or { lhs, rhs } => return Ok(binop_block(f, args, lhs, rhs, "value_or")?),
            Operator::Not { operand } => {
                let operand = compute_value(f, args, operand)?;
                writeln!(f, "value_not(&{operand});")?;
                return Ok(Return::Value(operand));
            }
        }
    }
}
