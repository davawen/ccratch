use std::io::{self, Write};

use nanoid::nanoid;
use crate::parser::{self, Block, Value, VarMap};

#[derive(Debug, Clone, Copy)]
enum ValueType {
    Num,
    Color,
    String,
    Bool
}

impl ValueType {
    /// The name of the enum value in the `ValueType` C enum
    fn enum_name(&self) -> &'static str {
        match self {
            Self::Num => "VALUE_NUM",
            Self::Color => "VALUE_COLOR",
            Self::String => "VALUE_STRING",
            Self::Bool => "VALUE_BOOL",
        }
    }

    /// The name of the member associated to this type of value in the `Value` struct.
    fn union_member(&self) -> char {
        match self {
            ValueType::Num => 'n',
            ValueType::Color => 'c',
            ValueType::String => 's',
            ValueType::Bool => 'b',
        }
    }
}

fn generate_var_name() -> String {
    pub const CIDENT: [char; 53] = [
        '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
        'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    nanoid!(12, &CIDENT)
}

/// get a variable in a function
fn get_var<'a>(args: &GeneratorArgs, id: &'_ str) -> String {
    if let Some(var) = args.target.vars.get(id) {
        format!("a->var_{var}")
    } else if let Some(var) = args.globals.get(id) {
        format!("g->var_{var}")
    } else {
        unreachable!("variable with ID `{id}` does not exists:\n{:#?}\n{:#?}", args.target.vars, args.globals);
    }
}

struct GeneratorArgs<'a> {
    target: &'a parser::Target,
    globals: &'a VarMap, 
    state: &'a mut u32,
    new_locals: &'a mut Vec<String>,
}

/// returns the variable name of the returned value
fn compute_value(
    f: &mut impl Write,
    args: &mut GeneratorArgs,
    value: &Value,
) -> io::Result<String> {
    let v = generate_var_name();
    match value {
        Value::Block(b) => {
            let v = linearize_block(f, args, &b)?;
            return Ok(v.expect("expected block to write to a variable"));
        }
        Value::Number(n) => {
            writeln!(f, "\t\tValue {v} = (Value){{ .type = VALUE_NUM, .n = {n} }};")?;
        }
        Value::Color(c) => {
            writeln!(
                f,
                "\t\tValue {v} = (Value){{ .type = VALUE_COLOR, .c = (ValueColor){{ {}, {}, {} }} }};",
                c.0, c.1, c.2
            )?;
        }
        Value::String(s) => {
            writeln!(f, "\t\tValue {v} = (Value){{ .type = VALUE_STRING, .s = \"{s}\" }};")?;
        }
        Value::Broadcast(_b) => {
            todo!()
        }
        Value::Variable(var) => {
            writeln!(f, "\t\tValue {} = {}; // {}", v, get_var(args, &var.id), var.name)?;
        }
        Value::List(_l) => {
            todo!()
        }
    }
    Ok(v)
}

/// `mapping_func` is a function that will be applied to both operands.
/// `op` is the operation that will be applied to the values.
/// `out_type` is the output type of the operation.
///
/// Returns a variable, with type `Value` but whose type will always be `out_type`.
fn binop_block(
    f: &mut impl Write,
    args: &mut GeneratorArgs,
    lhs: &Value,
    rhs: &Value,
    mapping_func: &str,
    op: &str,
    out_type: ValueType
) -> io::Result<String> {
    let lhs = compute_value(f, args, lhs)?;
    let rhs = compute_value(f, args, rhs)?;

    let member = out_type.union_member();

    writeln!(f, "\t\t{lhs}.{member} = {mapping_func}({lhs}) {op} {mapping_func}({rhs});")?;
    writeln!(f, "\t\t{lhs}.type = {};", out_type.enum_name())?;
    Ok(lhs)
}

/// returns the variable name of the returned value (if the block is an expression)
fn linearize_block(
    f: &mut impl Write,
    args: &mut GeneratorArgs,
    block: &Block,
) -> io::Result<Option<String>> {
    match block {
        Block::WhenFlagClicked => {
            writeln!(f, "\t\tif (g->flag_clicked) s->state = {};", *args.state + 1)?;
            return Ok(None);
        }
        Block::CreateCloneOf { actor } => { // TODO: 
            let _actor = compute_value(f, args, actor);
            writeln!(f, "\t\tprintf(\"TODO: create clone code\");")?;
            writeln!(f, "\t\texit(-1);")?;
        }
        Block::SetVariableTo { value, var } => {
            let value = compute_value(f, args, value)?;
            writeln!(
                f,
                "\t\t{} = {}; // setting {}",
                get_var(args, &var.id), value, var.name
            )?;
        }
        Block::Repeat { times, branch } => {
            *args.state += 1; // make the branch think we ended this case
            let repeat_start = *args.state;
            let mut branch_code = Vec::new();
            // prepare branch to get end state number
            linearize_sequence(&mut branch_code, args, branch)?;

            // initialize loop value (evaluate condition once)
            let num = compute_value(f, args, times)?;
            writeln!(f, "\t\tconvert_to_number(&{num})")?;

            // if initial condition is false, skip loop body
            writeln!(f, "\t\tif ((int){num}.n <= 0) s->state = {};", *args.state + 1)?;

            // otherwise, initialize loop variable and go to the next state (start of the sequence we linearized above)
            writeln!(f, "\t\telse {{")?;
            writeln!(f, "\t\t\ts->state = {};", repeat_start)?;
            let loop_var = format!("loop_{}", generate_var_name());
            writeln!(f, "\t\t\ts->{loop_var} = {num}.n;")?;
            writeln!(f, "\t\t}}")?;

            *args.state -= 1; // reverse previous add
            // end loop start
            end_case(f, args.state)?;

            f.write_all(&branch_code)?;

            // end loop (loop back condition)
            start_case(f, args.state)?;
            writeln!(f, "\t\ts->{loop_var}--;")?;
            writeln!(f, "\t\tif (s->{loop_var} > 0) s->state = {};", repeat_start)?;
            writeln!(f, "\t\telse s->state = {};", *args.state + 1)?;
            args.new_locals.push(loop_var);
            return Ok(None);
        }
        Block::IfCondition { condition, branch } => {
            *args.state += 1; // make the branch think we ended this case
            let branch_start = *args.state;
            let mut branch_code = Vec::new();
            // prepare branch to get end state number
            linearize_sequence(&mut branch_code, args, branch)?;

            let condition = compute_value(f, args, condition)?;
            writeln!(f, "\t\tconvert_to_bool(&{condition});")?;
            writeln!(f, "\t\tif ({condition}.b) s->state = {branch_start};")?;
            writeln!(f, "\t\telse s->state = {};", *args.state + 1)?;

            *args.state -= 1; // reverse previous add
            // end condition start
            end_case(f, args.state)?;
            f.write_all(&branch_code)?;

            start_case(f, args.state)?; // start a new case to counter act the end case automatically added
        }
        Block::SayForSecs { message, secs } => {
            let message = compute_value(f, args, &message)?;

            writeln!(f, "\t\tif ({message}.type == VALUE_NUM) printf(\"%f\\n\", {message}.n);")?;
            writeln!(f, "\t\telse if ({message}.type == VALUE_STRING) printf(\"%s\\n\", {message}.s);")?;
            writeln!(f, "\t\telse if ({message}.type == VALUE_COLOR) printf(\"#%02X%02X%02X\\n\", {message}.c.r, {message}.c.g, {message}.c.b);")?;
            writeln!(f, "\t\telse if ({message}.type == VALUE_BOOL) {{")?;
            writeln!(f, "\t\t\tif ({message}.b) printf(\"true\\n\");")?;
            writeln!(f, "\t\t\telse printf(\"false\\n\");")?;
            writeln!(f, "\t\t}}")?;
        }
        Block::CreateCloneOfMenu { actor } => {
            let v = generate_var_name();
            writeln!(
                f,
                "\t\tValue {v} = (Value){{ .type = VALUE_STRING, .s = \"{actor}\" }};"
            )?;
            return Ok(Some(v));
        }
        Block::Add { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "+", ValueType::Num)?)),
        Block::Sub { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "-", ValueType::Num)?)),
        Block::Mul { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "*", ValueType::Num)?)),
        Block::Div { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "/", ValueType::Num)?)),
        Block::GreaterThan { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", ">", ValueType::Bool)?)),
        Block::LesserThan { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "<", ValueType::Bool)?)),
        Block::Equals { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_number", "==", ValueType::Bool)?)),
        Block::And { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_bool", "&&", ValueType::Bool)?)),
        Block::Or { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_as_bool", "||", ValueType::Bool)?)),
        Block::Not { operand } => {
            let operand = compute_value(f, args, operand)?;
            writeln!(f, "\t\tconvert_to_bool(&{operand});")?;
            writeln!(f, "\t\t{operand}.b = !{operand}.b;")?;
            return Ok(Some(operand));
        }
    }
    writeln!(f, "\t\ts->state = {};", *args.state + 1)?;

    Ok(None)
}

fn start_case(f: &mut impl Write, state: &mut u32) -> io::Result<()> {
    writeln!(f, "\tcase {}: {{", *state)
}

fn end_case(f: &mut impl Write, state: &mut u32) -> io::Result<()> {
    writeln!(f, "\t}}")?;
    writeln!(f, "\tbreak;")?;
    *state += 1;
    Ok(())
}

fn linearize_sequence(
    f: &mut impl Write,
    args: &mut GeneratorArgs,
    sequence: &parser::Sequence,
) -> io::Result<()> {
    for block in &sequence.0 {
        start_case(f, args.state)?;
        linearize_block(f, args, block)?;
        end_case(f, args.state)?;
    }

    Ok(())
}

fn linearize(f: &mut impl Write, target: &parser::Target, globals: &VarMap, sequence: &parser::Sequence) -> io::Result<()> {
    let sequence_name = generate_var_name();

    let mut code_output = Vec::new();
    let mut state = 0;
    let mut new_locals = Vec::new();
    let mut args = GeneratorArgs {
        state: &mut state,
        new_locals: &mut new_locals,
        globals,
        target
    };
    linearize_sequence(&mut code_output, &mut args, sequence)?;

    writeln!(f, "typedef struct {{")?;
    writeln!(f, "\tint state;")?;
    writeln!(f, "\tfloat time;")?;
    for local in new_locals {
        writeln!(f, "\tint {local};")?;
    }
    writeln!(f, "}} Sequence{sequence_name}State;")?;
    writeln!(f)?;

    writeln!(
        f,
        "void sequence{}(Actor{} *a, Sequence{}State *s, GlobalState *g) {{",
        sequence_name, target.name, sequence_name
    )?;
    writeln!(f, "\tswitch (s->state) {{")?;
    f.write_all(&code_output)?;
    writeln!(f, "\t}}")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn generate_target(f: &mut impl Write, target: &parser::Target, globals: &VarMap) -> io::Result<()> {
    writeln!(f, "typedef struct {{")?;
    for (_, v) in &target.vars {
        writeln!(f, "\tValue var_{v};")?;
    }
    writeln!(f, "}} Actor{};", target.name)?;
    writeln!(f)?;
    for sequence in &target.code {
        linearize(f, target, globals, sequence)?;
    }
    writeln!(f)?;

    Ok(())
}

pub fn generate(out: &mut impl Write, targets: &[parser::Target], globals: &VarMap) -> io::Result<()> {
    writeln!(out, "#include <stdio.h>")?;
    writeln!(out, "#include \"runtime.h\"")?;
    writeln!(out)?;

    writeln!(out, "typedef struct {{")?;
    writeln!(out, "\tbool flag_clicked;")?;
    for global in globals.values() {
        writeln!(out, "\tValue var_{global};")?;
    }
    writeln!(out, "}} GlobalState;")?;

    for target in targets {
        generate_target(out, target, &globals)?;
    }

    Ok(())
}
