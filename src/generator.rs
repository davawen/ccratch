use std::io::{self, Write};

use crate::parser::{self, Block, Value, VarMap};
use nanoid::nanoid;

struct IndentWriter<W: Write> {
    writer: W,
    indent_level: u32,
    last_write_ended_with_newline: bool
}

impl<W: Write> Write for IndentWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for line in buf.split_inclusive(|&c| c == 0xa) { // split on newlines
            if self.last_write_ended_with_newline {
                for _ in 0..self.indent_level {
                    write!(self.writer, "    ")?;
                }
                self.last_write_ended_with_newline = false;
            }
            self.writer.write_all(line)?;

            if line.last() == Some(&0xa) {
                self.last_write_ended_with_newline = true;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> IndentWriter<W> {
    fn new(writer: W) -> Self {
        IndentWriter { writer, indent_level: 0, last_write_ended_with_newline: true }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn deindent(&mut self) {
        self.indent_level -= 1;
    }
}

type IW<W> = IndentWriter<W>;

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
        unreachable!(
            "variable with ID `{id}` does not exists:\n{:#?}\n{:#?}",
            args.target.vars, args.globals
        );
    }
}

struct GeneratorArgs<'a> {
    target: &'a parser::Target,
    globals: &'a VarMap,
    state: &'a mut u32,
    new_locals: &'a mut Vec<String>,
}

/// returns the variable name of the returned value
fn compute_value<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, value: &Value) -> io::Result<String> {
    let v = generate_var_name();
    match value {
        Value::Block(b) => {
            let v = linearize_block(f, args, &b)?;
            return Ok(v.expect("expected block to write to a variable"));
        }
        Value::Number(n) => {
            writeln!(f, "Value {v} = (Value){{ .type = VALUE_NUM, .n = {n} }};")?;
        }
        Value::Color(c) => {
            writeln!(
                f,
                "Value {v} = (Value){{ .type = VALUE_COLOR, .c = (ValueColor){{ {}, {}, {} }} }};",
                c.0, c.1, c.2
            )?;
        }
        Value::String(s) => {
            writeln!(f, "Value {v} = (Value){{ .type = VALUE_STRING, .s = create_rcstr(\"{s}\") }};")?;
        }
        Value::Broadcast(_b) => {
            todo!()
        }
        Value::Variable(var) => {
            writeln!(f, "Value {} = {}; // {}", v, get_var(args, &var.id), var.name)?;
        }
        Value::List(_l) => {
            todo!()
        }
    }
    Ok(v)
}

/// `op_func` is the function that will get applied to both operands.
///
/// Returns a variable's identifier.
fn binop_block<W: Write>(
    f: &mut IW<W>,
    args: &mut GeneratorArgs,
    lhs: &Value,
    rhs: &Value,
    op_func: &str,
) -> io::Result<String> {
    let lhs = compute_value(f, args, lhs)?;
    let rhs = compute_value(f, args, rhs)?;

    writeln!(f, "{lhs} = {op_func}({lhs}, {rhs});")?;
    Ok(lhs)
}

/// returns the variable name of the returned value (if the block is an expression)
fn linearize_block<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, block: &Block) -> io::Result<Option<String>> {
    match block {
        Block::WhenFlagClicked => {
            writeln!(f, "if (g->flag_clicked) s->state = {};", *args.state + 1)?;
            return Ok(None);
        }
        Block::MoveSteps { steps } => {
            let steps = compute_value(f, args, steps)?;
            writeln!(f, "convert_to_number(&{steps});")?;
            writeln!(f, "float direction = scratch_degrees_to_radians(a->actor_state.direction);")?;
            writeln!(f, "a->actor_state.x += cosf(direction)*{steps}.n;")?;
            writeln!(f, "a->actor_state.y += sinf(direction)*{steps}.n;")?;
        }
        Block::CreateCloneOf { actor } => {
            // TODO:
            let _actor = compute_value(f, args, actor);
            writeln!(f, "printf(\"TODO: create clone code\");")?;
            writeln!(f, "exit(-1);")?;
        }
        Block::SetVariableTo { value, var } => {
            let value = compute_value(f, args, value)?;
            writeln!(f, "{} = {};", get_var(args, &var.id), value)?;
        }
        Block::Wait { duration } => {
            let duration = compute_value(f, args, duration)?;
            writeln!(f, "convert_to_number(&{duration});")?;
            writeln!(f, "s->time = GetTime() + {duration}.n;")?;
            writeln!(f, "s->state = {};", *args.state + 1)?;

            end_case(f, args.state)?;
            start_case(f, args.state)?;

            writeln!(f, "if (GetTime() >= s->time) s->state = {};", *args.state + 1)?;
            return Ok(None);
        }
        Block::Repeat { times, branch } => {
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
            return Ok(None);
        }
        Block::IfCondition { condition, branch } => {
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

            start_case(f, args.state)?; // start a new case to counteract the automatically added end case
        }
        Block::SayForSecs { message, secs } => {
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
            return Ok(None);
        }
        Block::CreateCloneOfMenu { actor } => {
            let v = generate_var_name();
            writeln!(
                f,
                "Value {v} = (Value){{ .type = VALUE_STRING, .s = \"{actor}\" }};"
            )?;
            return Ok(Some(v));
        }
        Block::Add { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_add")?)),
        Block::Sub { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_sub")?)),
        Block::Mul { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_mul")?)),
        Block::Div { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_div")?)),
        Block::GreaterThan { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_greater_than")?)),
        Block::LesserThan { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_lesser_than")?)),
        Block::Equals { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_equal")?)),
        Block::And { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_and")?)),
        Block::Or { lhs, rhs } => return Ok(Some(binop_block(f, args, lhs, rhs, "value_or")?)),
        Block::Not { operand } => {
            let operand = compute_value(f, args, operand)?;
            writeln!(f, "value_not(&{operand});")?;
            return Ok(Some(operand));
        }
    }
    writeln!(f, "s->state = {};", *args.state + 1)?;

    Ok(None)
}

fn start_case<W: Write>(f: &mut IW<W>, state: &mut u32) -> io::Result<()> {
    writeln!(f, "case {}: {{", *state)?;
    f.indent();
    Ok(())
}

fn end_case<W: Write>(f: &mut IW<W>, state: &mut u32) -> io::Result<()> {
    f.deindent();
    writeln!(f, "}}")?;
    writeln!(f, "break;")?;
    *state += 1;
    Ok(())
}

fn linearize_sequence<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, sequence: &parser::Sequence) -> io::Result<()> {
    for block in &sequence.0 {
        start_case(f, args.state)?;
        linearize_block(f, args, block)?;
        end_case(f, args.state)?;
    }

    Ok(())
}

fn linearize<W: Write>(
    header: &mut IW<W>,
    source: &mut IW<W>,
    target: &parser::Target,
    globals: &VarMap,
    sequence_index: usize,
) -> io::Result<()> {
    let mut state = 0;
    let mut new_locals = Vec::new();
    let mut args = GeneratorArgs {
        state: &mut state,
        new_locals: &mut new_locals,
        globals,
        target,
    };
    writeln!(source, "/// Returns wether the sequence has finished running.")?;
    writeln!(
        source,
        "bool run_{}_sequence{sequence_index}(Actor{} *a, {}Sequence{sequence_index}State *s, GlobalState *g) {{",
        target.name, target.name, target.name
    )?;
    source.indent();
    writeln!(source, "switch (s->state) {{")?;
    linearize_sequence(source, &mut args, &target.sequences[sequence_index])?;
    start_case(source, &mut state)?;
    writeln!(source, "return true;")?;
    end_case(source, &mut state)?;
    writeln!(source, "}}")?;
    writeln!(source, "return false;")?;
    source.deindent();
    writeln!(source, "}}")?;

    writeln!(header, "typedef struct {{")?;
    source.indent();
    writeln!(header, "int state;")?;
    writeln!(header, "float time;")?;
    for local in new_locals {
        writeln!(header, "int {local};")?;
    }
    source.deindent();
    writeln!(header, "}} {}Sequence{sequence_index}State;", target.name)?;
    writeln!(header)?;

    Ok(())
}

fn generate_target<W: Write>(header: &mut IW<W>, source: &mut IW<W>, target: &parser::Target, globals: &VarMap) -> io::Result<()> {
    for i in 0..target.sequences.len() {
        linearize(header, source, target, globals, i)?;
    }

    match &target.kind {
        parser::TargetKind::Stage { .. } => {
            writeln!(header, "typedef struct {{")?;
            header.indent();
            writeln!(header, "int current_backdrop;")?;
            writeln!(header, "Sprite *backdrops;")?;
            writeln!(header, "int tempo;")?;
            for i in 0..target.sequences.len() {
                writeln!(header, "StageSequence{i}State sequence{i}_state;")?;
            }
            header.deindent();
            writeln!(header, "}} ActorStage;")?;
        },
        parser::TargetKind::Sprite { .. } => {
            writeln!(header, "typedef struct {{")?;
            header.indent();
            writeln!(header, "ActorState actor_state;")?;
            for (_, v) in &target.vars {
                writeln!(header, "Value var_{v};")?;
            }
            for i in 0..target.sequences.len() {
                writeln!(header, "{}Sequence{i}State sequence{i}_state;", target.name)?;
            }
            header.deindent();
            writeln!(header, "}} Actor{};", target.name)?;
        },
    }
    writeln!(header)?;

    writeln!(source, "/// update every sequence of this actor")?;
    writeln!(
        source,
        "void run_{}(Actor{} *a, GlobalState *g) {{",
        target.name, target.name
    )?;
    source.indent();
    for i in 0..target.sequences.len() {
        writeln!(source, "run_{}_sequence{i}(a, &a->sequence{i}_state, g);", target.name)?;
    }
    source.deindent();
    writeln!(source, "}}")?;
    writeln!(source)?;

    writeln!(source, "Sprite sprites_{}[{}] = {{ 0 }};", target.name, target.costumes.len())?;
    writeln!(source, "void init_sprites_{}() {{", target.name)?;
    source.indent();
    for (i, costume) in target.costumes.iter().enumerate() {
        writeln!(source, "// Load {}", costume.name)?;
        writeln!(source, "sprites_{}[{i}].rotation_center_x = {};", target.name, costume.rotation_center_x)?;
        writeln!(source, "sprites_{}[{i}].rotation_center_y = {};", target.name, costume.rotation_center_y)?;
        writeln!(source, "sprites_{}[{i}].texture = LoadTexture(\"project/{}\");", target.name, costume.filename)?;
    }
    source.deindent();
    writeln!(source, "}}")?;

    Ok(())
}

pub fn generate<W: Write>(header: &mut W, source: &mut W, targets: &[parser::Target], globals: &VarMap) -> io::Result<()> {
    let header = &mut IW::new(header);
    let source = &mut IW::new(source);

    writeln!(header, "#include <stdio.h>")?;
    writeln!(header, "#include <string.h>")?;
    writeln!(header, "#include <math.h>")?;
    writeln!(header, "#include \"runtime.h\"")?;
    writeln!(header)?;

    writeln!(source, "#include \"output.h\"")?;
    writeln!(source)?;

    for target in targets {
        generate_target(header, source, target, &globals)?;
    }

    writeln!(header, "typedef struct {{")?;
    header.indent();

    writeln!(header, "bool flag_clicked;")?;
    for global in globals.values() {
        writeln!(header, "Value var_{global};")?;
    }
    writeln!(header, "ActorStage stage;")?;
    for target in targets {
        if matches!(target.kind, parser::TargetKind::Stage { .. }) { continue }

        writeln!(header, "int num_{};", target.name)?;
        writeln!(header, "Actor{} *list_{};", target.name, target.name)?;
    }

    header.deindent();
    writeln!(header, "}} GlobalState;")?;
    writeln!(header)?;

    generate_global_functions(header, source, targets, globals)?;

    Ok(())
}

fn generate_global_functions<W: Write>(header: &mut IW<W>, source: &mut IW<W>, targets: &[parser::Target], globals: &VarMap) -> io::Result<()> {
    writeln!(header, "GlobalState init_global();")?;
    writeln!(header, "void run_global(GlobalState *g);")?;
    writeln!(header, "void render_global(GlobalState *g);")?;

    writeln!(source, "/// initialize every scratch object (raylib needs to be initialized first)")?;
    writeln!(source, "GlobalState init_global() {{")?;

    source.indent();
    writeln!(source, "// Load every sprite")?;
    for target in targets {
        writeln!(source, "init_sprites_{}();", target.name)?;
    }
    writeln!(source)?;

    writeln!(source, "// Initialize every actor")?;
    for target in targets {
        match &target.kind {
            parser::TargetKind::Stage { tempo } => {
                writeln!(source, "ActorStage stage = {{")?;
                source.indent();
                writeln!(source, ".current_backdrop = {},", target.current_costume)?;
                writeln!(source, ".backdrops = sprites_Stage,")?;
                writeln!(source, ".tempo = {tempo},")?;
                for i in 0..target.sequences.len() {
                    writeln!(source, ".sequence{i}_state = (StageSequence{i}State) {{ 0 }},")?;
                }
                source.deindent();
                writeln!(source, "}};")?;
            },
            parser::TargetKind::Sprite { visible, x, y, size, direction, draggable: _, rotation_style: _ } => {
                let name = &target.name;
                writeln!(source, "Actor{name} *list_{name} = malloc(sizeof(Actor{name}));")?;
                // TODO: Initalize default variable value

                writeln!(source, "list_{name}->actor_state = (ActorState) {{")?;
                source.indent();
                writeln!(source, ".x = {x},")?;
                writeln!(source, ".y = {y},")?;
                writeln!(source, ".size = {size},")?;
                writeln!(source, ".direction = {direction},")?;
                writeln!(source, ".visible = {visible},")?;
                writeln!(source, ".sprite_index = {},", target.current_costume)?;
                writeln!(source, ".sprites = sprites_{name}")?;
                source.deindent();
                writeln!(source, "}};")?;
                for i in 0..target.sequences.len() {
                    writeln!(source, "list_{name}->sequence{i}_state = ({name}Sequence{i}State) {{ 0 }};")?;
                }
            },
        }
        writeln!(source)?;
    }

    writeln!(source, "// Initialize global state")?;
    writeln!(source, "return (GlobalState) {{")?;
    source.indent();

    writeln!(source, ".flag_clicked = false,")?;
    for var in globals.values() {
        // TODO: Initialize default variable value
        writeln!(source, ".var_{var} = (Value) {{ .type = VALUE_NUM, .n = 0 }},")?;
    }

    for target in targets {
        if matches!(target.kind, parser::TargetKind::Stage { .. }) {
            writeln!(source, ".stage = stage,")?;
        } else {
            let name = &target.name;
            writeln!(source, ".num_{name} = 1,")?;
            writeln!(source, ".list_{name} = list_{name},")?;
        }
    }

    source.deindent();
    writeln!(source, "}};")?;

    source.deindent();
    writeln!(source, "}}")?;
    writeln!(source)?;

    writeln!(source, "void run_global(GlobalState *g) {{")?;
    source.indent();
    for target in targets {
        match target.kind {
            parser::TargetKind::Stage { .. } => {
                for i in 0..target.sequences.len() {
                    writeln!(source, "run_Stage_sequence{i}(&g->stage, &g->stage.sequence{i}_state, g);")?;
                }
            }
            parser::TargetKind::Sprite { .. } => {
                let name = &target.name;
                writeln!(source, "for (int i = 0; i < g->num_{name}; i++) {{")?;
                source.indent();
                writeln!(source, "Actor{name} *a = &g->list_{name}[i];")?;
                for i in 0..target.sequences.len() {
                    writeln!(source, "run_{name}_sequence{i}(a, &a->sequence{i}_state, g);")?;
                }
                source.deindent();
                writeln!(source, "}}")?;
            }
        }
        writeln!(source)?;
    }
    source.deindent();
    writeln!(source, "}}")?;
    writeln!(source)?;

    writeln!(source, "void render_global(GlobalState *g) {{")?;
    source.indent();

    // always draw stage first
    writeln!(source, "DrawTexture(g->stage.backdrops[g->stage.current_backdrop].texture, 0, 0, WHITE);")?;

    for target in targets {
        if matches!(target.kind, parser::TargetKind::Stage { .. }) { continue }

        let name = &target.name;
        writeln!(source, "for (int i = 0; i < g->num_{name}; i++) {{")?;
        source.indent();
        writeln!(source, "draw_actor(&g->list_{name}[i].actor_state);")?;
        source.deindent();
        writeln!(source, "}}")?;
        writeln!(source)?;
    }
    source.deindent();
    writeln!(source, "}}")?;

    Ok(())
}

