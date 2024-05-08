use std::collections::HashMap;

mod parser;

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Broadcast {
    name: String,
    id: String,
}

#[derive(Debug)]
struct Variable {
    name: String,
    id: String,
}

#[derive(Debug)]
struct List {
    name: String,
    id: String,
}

#[derive(Debug)]
enum Value {
    Block(Box<Block>),
    Number(f64),
    Positive(f64),
    PositiveInt(u64),
    Int(i64),
    Angle(i64),
    Color(Color),
    String(String),
    Broadcast(Broadcast),
    Variable(Variable),
    List(List),
}

#[derive(Debug)]
enum Block {
    Nop,
    WhenFlagClicked,
    SetVariableTo { value: Value, var: Variable },
    Repeat { times: Value, branch: Vec<Block> },
    Add { lhs: Value, rhs: Value },
    SayForSecs { message: Value, secs: Value },
}

#[derive(Debug)]
struct Sequence {
    start: Block,
    sequence: Vec<Block>,
}

fn parse_field_variable(v: &(serde_json::Value, Option<serde_json::Value>)) -> Variable {
    Variable {
        name: v.0.as_str().unwrap().to_owned(),
        id: v.1.as_ref().unwrap().as_str().unwrap().to_owned(),
    }
}

/// Parses an RGB color code formatted like `#RRGGBB` into the `Color` struct.
fn parse_color(hexadecimal_code: &str) -> Color {
    let r = u8::from_str_radix(&hexadecimal_code[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hexadecimal_code[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hexadecimal_code[5..7], 16).unwrap();
    Color(r, g, b)
}

fn parse_value(blocks: &HashMap<String, parser::Block>, v: &[serde_json::Value]) -> Value {
    // The first element of `v` describes wether the input is shadowed or not
    // But, whether is is the case, the actual value will always be the second element in the array
    // So we don't care about the shadowed state

    if v[1].is_string() {
        let id = v[1].as_str().unwrap();
        Value::Block(Box::new(parse_block(blocks, &blocks[id])))
    } else if v[1].is_array() {
        let v = v[1].as_array().unwrap();

        // The first value tells us what type of element it is
        let kind = v[0].as_u64().unwrap();
        match kind {
            4 => Value::Number(v[1].as_str().unwrap().parse().unwrap()),
            5 => Value::Positive(v[1].as_str().unwrap().parse().unwrap()),
            6 => Value::PositiveInt(v[1].as_str().unwrap().parse().unwrap()),
            7 => Value::Int(v[1].as_str().unwrap().parse().unwrap()),
            8 => Value::Angle(v[1].as_str().unwrap().parse().unwrap()),
            9 => Value::Color(parse_color(v[1].as_str().unwrap())),
            10 => Value::String(v[1].as_str().unwrap().to_owned()),
            11 | 12 | 13 => {
                let name = v[1].as_str().unwrap().to_owned();
                let id = v[2].as_str().unwrap().to_owned();
                match kind {
                    11 => Value::Broadcast(Broadcast { name, id }),
                    12 => Value::Variable(Variable { name, id }),
                    13 => Value::List(List { name, id }),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!("invalid value: {kind}"),
        }
    } else {
        unreachable!("invalid block")
    }
}

fn parse_block(blocks: &HashMap<String, parser::Block>, block: &parser::Block) -> Block {
    match block.opcode.as_str() {
        "event_whenflagclicked" => Block::WhenFlagClicked,
        "looks_sayforsecs" => {
            let message = parse_value(blocks, &block.inputs["MESSAGE"]);
            let secs = parse_value(blocks, &block.inputs["SECS"]);
            Block::SayForSecs { message, secs }
        }
        "control_repeat" => {
            let times = parse_value(blocks, &block.inputs["TIMES"]);
            let branch = if let Some(id) = block.inputs["SUBSTACK"][1].as_str() {
                parse_block_list(blocks, &blocks[id], true)
            } else {
                vec![Block::Nop]
            };
            Block::Repeat { times, branch }
        }
        "control_create_clone_of" => Block::WhenFlagClicked,
        "data_setvariableto" => {
            let value = parse_value(blocks, &block.inputs["VALUE"]);
            let var = parse_field_variable(&block.fields["VARIABLE"]);
            Block::SetVariableTo { value, var }
        }
        "operator_add" => {
            let lhs = parse_value(blocks, &block.inputs["NUM1"]);
            let rhs = parse_value(blocks, &block.inputs["NUM2"]);
            Block::Add { lhs, rhs }
        }
        opcode => todo!("unimplemented block: {opcode}"),
    }
}

fn parse_block_list(
    blocks: &HashMap<String, parser::Block>,
    start: &parser::Block,
    include_start: bool,
) -> Vec<Block> {
    let mut sequence = vec![];
    if include_start {
        sequence.push(parse_block(blocks, start));
    }

    let mut block = start;
    while let Some(next_id) = &block.next {
        block = &blocks[next_id];

        sequence.push(parse_block(blocks, block));
    }

    sequence
}

fn parse_sequence(blocks: &HashMap<String, parser::Block>, start: &parser::Block) -> Sequence {
    Sequence {
        start: parse_block(blocks, start),
        sequence: parse_block_list(blocks, start, false),
    }
}

fn linearize(target: &parser::Target) -> Vec<Sequence> {
    let mut sequences = vec![];

    for (id, block) in &target.blocks {
        if block.topLevel {
            sequences.push(parse_sequence(&target.blocks, &block));
        }
    }

    sequences
}

fn main() {
    let data = std::fs::read_to_string("./project/project.json").unwrap();

    let v: parser::Project = serde_json::from_str(&data).unwrap();
    let v = v.targets;

    for target in v {
        let sequences = linearize(&target);
        println!("{sequences:#?}");
    }
}
