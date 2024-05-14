use std::io::{self, Write};

use nanoid::nanoid;
use parser::{Block, Value, VarMap};

mod parser;
mod scratch;

fn generate_var_name() -> String {
    pub const CIDENT: [char; 53] = [
        '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
        'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    nanoid!(12, &CIDENT)
}

/// get a variable in a function
fn get_var<'a>(target: &'a parser::Target, globals: &'a VarMap, id: &'_ str) -> String {
    if let Some(var) = target.vars.get(id) {
        format!("a->var_{var}")
    } else if let Some(var) = globals.get(id) {
        format!("g->var_{var}")
    } else {
        unreachable!("variable with ID `{id}` does not exists:\n{:#?}\n{globals:#?}", target.vars);
    }
}

/// returns the variable name of the returned value
fn compute_value(
    f: &mut impl Write,
    target: &parser::Target,
    globals: &VarMap, 
    state: &mut u32,
    new_locals: &mut Vec<String>,
    value: &Value,
) -> io::Result<String> {
    let v = generate_var_name();
    match value {
        Value::Block(b) => {
            let v = linearize_block(f, target, globals, state, new_locals, &b)?;
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
        Value::Broadcast(b) => {
            todo!()
        }
        Value::Variable(var) => {
            writeln!(f, "\t\tValue {} = {}; // {}", v, get_var(target, globals, &var.id), var.name)?;
        }
        Value::List(l) => {
            todo!()
        }
    }
    Ok(v)
}

fn arithmetic_block(
    f: &mut impl Write,
    target: &parser::Target,
    globals: &VarMap, 
    state: &mut u32,
    new_locals: &mut Vec<String>,
    lhs: &Value,
    rhs: &Value,
    op: &str
) -> io::Result<String> {
    let lhs = compute_value(f, target, globals, state, new_locals, lhs)?;
    let rhs = compute_value(f, target, globals, state, new_locals, rhs)?;

    writeln!(f, "\t\t{lhs}.n = value_as_number({lhs}) {op} value_as_number({rhs});")?;
    writeln!(f, "\t\t{lhs}.type = VALUE_NUM;")?;
    return Ok(lhs);
}

/// returns the variable name of the returned value (if the block is an expression)
fn linearize_block(
    f: &mut impl Write,
    target: &parser::Target,
    globals: &VarMap, 
    state: &mut u32,
    new_locals: &mut Vec<String>,
    block: &Block,
) -> io::Result<Option<String>> {
    match block {
        Block::WhenFlagClicked => {
            writeln!(f, "\t\tif (g->flag_clicked) s->state = {};", *state + 1)?;
            return Ok(None);
        }
        Block::CreateCloneOf { actor } => {
            let actor = compute_value(f, target, globals, state, new_locals, actor);
            writeln!(f, "\t\tprintf(\"TODO: create clone code\");")?;
            writeln!(f, "\t\texit(-1);")?;
        }
        Block::SetVariableTo { value, var } => {
            let value = compute_value(f, target, globals, state, new_locals, value)?;
            writeln!(
                f,
                "\t\t{} = {}; // setting {}",
                get_var(target, globals, &var.id), value, var.name
            )?;
        }
        Block::Repeat { times, branch } => {
            *state += 1; // make the branch think we ended this case
            let repeat_start = *state;
            let mut branch_code = Vec::new();
            // prepare branch to get end state number
            linearize_sequence(&mut branch_code, target, globals, state, new_locals, branch)?;

            // initialize loop value (evaluate condition once)
            let num = compute_value(f, target, globals, state, new_locals, times)?;
            writeln!(f, "\t\t{num}.n = value_as_number({num});")?;
            writeln!(f, "\t\t{num}.type = VALUE_NUM;")?;

            // if initial condition is false, skip loop body
            writeln!(f, "\t\tif ((int){num}.n <= 0) s->state = {};", *state + 1)?;
            writeln!(f, "\t\telse {{")?;

            writeln!(f, "\t\t\ts->state = {};", repeat_start)?;
            let loop_var = format!("loop_{}", generate_var_name());
            writeln!(f, "\t\t\ts->{loop_var} = {num}.n;")?;

            writeln!(f, "\t\t}}")?;

            *state -= 1; // reverse previous add
            // end loop start
            end_case(f, state)?;

            f.write_all(&branch_code)?;

            // end loop (loop back condition)
            start_case(f, state)?;
            writeln!(f, "\t\ts->{loop_var}--;")?;
            writeln!(f, "\t\tif (s->{loop_var} > 0) s->state = {};", repeat_start)?;
            writeln!(f, "\t\telse s->state = {};", *state + 1)?;
            new_locals.push(loop_var);
            return Ok(None);
        }
        Block::SayForSecs { message, secs } => {
            let message = compute_value(f, target, globals, state, new_locals, &message)?;

            writeln!(f, "\t\tif ({message}.type == VALUE_NUM) printf(\"%f\\n\", {message}.n);")?;
            writeln!(f, "\t\telse if ({message}.type == VALUE_STRING) printf(\"%s\\n\", {message}.s);")?;
            writeln!(f, "\t\telse if ({message}.type == VALUE_COLOR) printf(\"#%02X%02X%02X\\n\", {message}.c.r, {message}.c.g, {message}.c.b);")?;
        }
        Block::CreateCloneOfMenu { actor } => {
            let v = generate_var_name();
            writeln!(
                f,
                "\t\tValue {v} = (Value){{ .type = VALUE_STRING, .s = \"{actor}\" }};"
            )?;
            return Ok(Some(v));
        }
        Block::Add { lhs, rhs } => return Ok(Some(arithmetic_block(f, target, globals, state, new_locals, lhs, rhs, "+")?)),
        Block::Sub { lhs, rhs } => return Ok(Some(arithmetic_block(f, target, globals, state, new_locals, lhs, rhs, "-")?)),
        Block::Mul { lhs, rhs } => return Ok(Some(arithmetic_block(f, target, globals, state, new_locals, lhs, rhs, "*")?)),
        Block::Div { lhs, rhs } => return Ok(Some(arithmetic_block(f, target, globals, state, new_locals, lhs, rhs, "/")?)),
    }
    writeln!(f, "\t\ts->state = {};", *state + 1)?;

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
    target: &parser::Target,
    globals: &VarMap, 
    state: &mut u32,
    new_locals: &mut Vec<String>,
    sequence: &parser::Sequence,
) -> io::Result<()> {
    for block in &sequence.0 {
        start_case(f, state)?;
        linearize_block(f, target, globals, state, new_locals, block)?;
        end_case(f, state)?;
    }

    Ok(())
}

fn linearize(f: &mut impl Write, target: &parser::Target, globals: &VarMap, sequence: &parser::Sequence) -> io::Result<()> {
    let sequence_name = generate_var_name();

    let mut code_output = Vec::new();
    let mut state = 0;
    let mut new_locals = Vec::new();
    linearize_sequence(&mut code_output, target, globals, &mut state, &mut new_locals, sequence)?;

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

fn generate(f: &mut impl Write, target: &parser::Target, globals: &VarMap) -> io::Result<()> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("./project/project.json")?;

    let project: scratch::Project = serde_json::from_str(&data)?;
    let targets = project.targets;
    let (targets, globals) = parser::parse(targets);

    let mut out = std::fs::File::create("output.c")?;
    writeln!(out, "#include <stdio.h>")?;
    writeln!(out, "#include \"runtime.h\"")?;
    writeln!(out)?;

    writeln!(out, "typedef struct {{")?;
    writeln!(out, "\tbool flag_clicked;")?;
    for global in globals.values() {
        writeln!(out, "\tValue var_{global};")?;
    }
    writeln!(out, "}} GlobalState;")?;

    for target in &targets {
        generate(&mut out, target, &globals)?;
    }

    Ok(())
}
