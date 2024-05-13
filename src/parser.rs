use crate::scratch;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Broadcast {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub struct List {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub enum Value {
    Block(Box<Block>),
    Number(f64),
    Color(Color),
    String(String),
    Broadcast(Broadcast),
    Variable(Variable),
    List(List),
}

#[derive(Debug)]
pub enum Block {
    WhenFlagClicked,
    CreateCloneOf { actor: Value },
    CreateCloneOfMenu { actor: String },
    SetVariableTo { value: Value, var: Variable },
    Repeat { times: Value, branch: Sequence },
    Add { lhs: Value, rhs: Value },
    SayForSecs { message: Value, secs: Value },
}

#[derive(Debug)]
pub struct Sequence(pub Vec<Block>);

#[derive(Debug)]
pub struct Target {
    pub name: String,
    pub code: Vec<Sequence>,
    /// Map from variable ID to C variable name
    pub vars: HashMap<String, String>,
    pub costumes: Vec<scratch::Costume>,
    pub sounds: Vec<scratch::Sound>,
}

fn parse_field_clone_option(v: &(serde_json::Value, Option<serde_json::Value>)) -> String {
    v.0.as_str().unwrap().to_owned()
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

fn parse_value(blocks: &HashMap<String, scratch::Block>, v: &[serde_json::Value]) -> Value {
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
            4..=8 => Value::Number(v[1].as_str().unwrap().parse().unwrap()),
            9 => Value::Color(parse_color(v[1].as_str().unwrap())),
            10 => {
                let v = v[1].as_str().unwrap();
                match v.parse() {
                    Ok(n) => Value::Number(n),
                    Err(_) => Value::String(v.to_owned()),
                }
            }
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

fn parse_block(blocks: &HashMap<String, scratch::Block>, block: &scratch::Block) -> Block {
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
                parse_sequence(blocks, &blocks[id])
            } else {
                Sequence(vec![])
            };
            Block::Repeat { times, branch }
        }
        "control_create_clone_of" => {
            let actor = parse_value(blocks, &block.inputs["CLONE_OPTION"]);
            Block::CreateCloneOf { actor }
        }
        "control_create_clone_of_menu" => {
            let actor = parse_field_clone_option(&block.fields["CLONE_OPTION"]);
            Block::CreateCloneOfMenu { actor }
        }
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

fn parse_sequence(blocks: &HashMap<String, scratch::Block>, start: &scratch::Block) -> Sequence {
    let mut sequence = vec![];
    sequence.push(parse_block(blocks, start));

    let mut block = start;
    while let Some(next_id) = &block.next {
        block = &blocks[next_id];

        sequence.push(parse_block(blocks, block));
    }

    Sequence(sequence)
}

pub type VarMap = HashMap<String, String>;

/// transforms scratch variable names into valid C identifiers
/// ascii alphanumeric characters are passed through,
/// other characters are transformed into underscores,
/// conflicts are resolved by adding underscores at the end of the word
///
/// only local variables are returned
/// `globals` is another hash map created to gather global variables
fn sanitize_varnames(vars: HashMap<String, scratch::Variable>, globals: &mut VarMap) -> VarMap {
    let mut out = HashMap::new();
    let mut cidents = HashSet::new();

    for (id, var) in vars {
        let mut cident = var.0.replace(|x: char| !x.is_ascii_alphanumeric(), "_");

        if var.2.is_some_and(|global| global) {
            // FIXME: horibly inneficient, should do a global hash set
            while globals.values().find(|g| g == &&cident).is_some() {
                cident.push('_');
            }
            globals.insert(id, cident);
            continue
        }

        while cidents.contains(&cident) {
            cident.push('_');
        }
        cidents.insert(cident.clone());
        out.insert(id, cident);
    }

    out
}

fn parse_target(target: scratch::Target, globals: &mut VarMap) -> Target {
    let mut sequences = vec![];

    for (_, block) in &target.blocks {
        if block.topLevel {
            sequences.push(parse_sequence(&target.blocks, &block));
        }
    }

    Target {
        name: target.name,
        code: sequences,
        vars: sanitize_varnames(target.variables, globals),
        costumes: target.costumes,
        sounds: target.sounds,
    }
}

pub fn parse(targets: Vec<scratch::Target>) -> (Vec<Target>, VarMap) {
    let mut globals = VarMap::new();

    let targets = targets.into_iter().map(|t| parse_target(t, &mut globals)).collect();
    (targets, globals)
}
