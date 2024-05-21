use std::io::{self, Write};

use crate::parser::{self, Block, Value, VarMap};

mod util;
use util::*;

mod motion;
mod looks;
mod event;
mod control;
mod operator;
mod data;

/// returns the variable name of the returned value
fn compute_value<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, value: &Value) -> io::Result<String> {
    let v = generate_var_name();
    match value {
        Value::Block(b) => {
            let v = linearize_block(f, args, &b)?;
            match v {
                Return::Value(v) => return Ok(v),
                _ => unreachable!("expected a block that returns a value:\n{b:#?}")
            }
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

trait Linearize {
    fn linearize<W: Write>(&self, f: &mut IW<W>, args: &mut GeneratorArgs) -> io::Result<Return>;
}

/// returns the variable name of the returned value (if the block is an expression)
fn linearize_block<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, block: &Block) -> io::Result<Return> {
    let ret = match block {
        Block::Motion(v) => v.linearize(f, args),
        Block::Looks(v) => v.linearize(f, args),
        Block::Event(v) => v.linearize(f, args),
        Block::Control(v) => v.linearize(f, args),
        Block::Operator(v) => v.linearize(f, args),
        Block::Data(v) => v.linearize(f, args),
    }?;

    match ret {
        Return::Empty => writeln!(f, "s->state = {};", *args.state + 1)?,
        Return::Value(_) => unreachable!("block returning a value used as a statement:\n{block:#?}"),
        _ => ()
    }
    Ok(ret)
}

fn linearize_sequence<W: Write>(f: &mut IW<W>, args: &mut GeneratorArgs, sequence: &parser::Sequence) -> io::Result<()> {
    for block in &sequence.0 {
        start_case(f, args.state)?;
        let ret = linearize_block(f, args, block)?;
        match ret {
            Return::Empty | Return::Hold => end_case(f, args.state)?,
            Return::Value(_) => unreachable!(),
            Return::Ended => ()
        }
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

