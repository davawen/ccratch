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
pub enum Motion {
    MoveSteps { steps: Value },
    TurnRight { degrees: Value },
    TurnLeft { degrees: Value },
    Goto { dest: GotoDestOption },
    GotoXY { x: Value, y: Value },
    Glide { secs: Value, dest: GotoDestOption },
    GlideXY { secs: Value, x: Value, y: Value },
    PointInDrection { degrees: Value },
    PointTowards { towards: PointTowardsOption },
    ChangeX { by: Value },
    SetX { to: Value },
    ChangeY { by: Value },
    SetY { to: Value },
    IfOnEdgeBounce,
    SetRotationStyle { style: RotationStyle },

    // Value producing blocks
    XPosition,
    YPosition,
    Direction
}

#[derive(Debug)]
pub enum Looks {
    SayForSecs { message: Value, secs: Value },
}

#[derive(Debug)]
pub enum Event {
    WhenFlagClicked,
}

#[derive(Debug)]
pub enum Control {
    Wait { duration: Value },
    Repeat { times: Value, branch: Sequence },
    IfCondition { condition: Value, branch: Sequence },
    CreateCloneOf { actor: String },
}

#[derive(Debug)]
pub enum Operator {
    Add { lhs: Value, rhs: Value },
    Sub { lhs: Value, rhs: Value },
    Mul { lhs: Value, rhs: Value },
    Div { lhs: Value, rhs: Value },
    GreaterThan { lhs: Value, rhs: Value },
    LesserThan { lhs: Value, rhs: Value },
    Equals { lhs: Value, rhs: Value },
    And { lhs: Value, rhs: Value },
    Or { lhs: Value, rhs: Value },
    Not { operand: Value },
}

#[derive(Debug)]
pub enum Data {
    SetVariableTo { value: Value, var: Variable },
}

#[derive(Debug)]
pub enum Block {
    Motion(Motion),
    Looks(Looks),
    Event(Event),
    Control(Control),
    Operator(Operator),
    Data(Data)
}

impl From<Motion> for Block { fn from(value: Motion) -> Self { Block::Motion(value) } }
impl From<Looks> for Block { fn from(value: Looks) -> Self { Block::Looks(value) } }
impl From<Event> for Block { fn from(value: Event) -> Self { Block::Event(value) } }
impl From<Control> for Block { fn from(value: Control) -> Self { Block::Control(value) } }
impl From<Operator> for Block { fn from(value: Operator) -> Self { Block::Operator(value) } }
impl From<Data> for Block { fn from(value: Data) -> Self { Block::Data(value) } }

#[derive(Debug)]
pub struct Sequence(pub Vec<Block>);

#[derive(Debug)]
pub struct Costume {
    pub name: String,
    pub filename: String,
    pub bitmap_resolution: i32,
    pub rotation_center_x: i32,
    pub rotation_center_y: i32,
}

#[derive(Debug)]
pub enum GotoDestOption {
    Random,
    MouseCursor,
    Actor(String)
}

#[derive(Debug)]
pub enum PointTowardsOption {
    MouseCursor,
    Actor(String)
}

#[derive(Debug)]
pub enum RotationStyle {
    AllAround,
    LeftRight,
    DontRotate
}

#[derive(Debug)]
pub enum TargetKind {
    Stage { tempo: u32 },
    Sprite {
        visible: bool,
        x: f32,
        y: f32,
        size: f32,
        direction: f32,
        draggable: bool,
        rotation_style: RotationStyle
    }
}

#[derive(Debug)]
pub struct Target {
    pub name: String,
    pub sequences: Vec<Sequence>,
    /// Map from variable ID to C variable name
    pub vars: HashMap<String, String>,
    pub current_costume: usize,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<scratch::Sound>,
    pub kind: TargetKind
}

fn parse_clone_option(blocks: &HashMap<String, scratch::Block>, v: &[serde_json::Value]) -> String {
    let id = v[1].as_str().expect("expected clone input option to point to a block");
    let block = &blocks[id];
    assert!(block.opcode == "control_create_clone_of_menu");

    block.fields["CLONE_OPTION"].0.as_str().unwrap().to_owned()
}

fn parse_goto_option(blocks: &HashMap<String, scratch::Block>, v: &[serde_json::Value]) -> GotoDestOption {
    let id = v[1].as_str().expect("expected goto input option to point to a block");
    let block = &blocks[id];
    assert!(block.opcode == "motion_goto_menu" || block.opcode == "motion_glideto_menu");

    match block.fields["TO"].0.as_str().unwrap() {
        "_random_" => GotoDestOption::Random,
        "_mouse_" => GotoDestOption::MouseCursor,
        s => GotoDestOption::Actor(s.to_owned())
    }
}

fn parse_towards_option(blocks: &HashMap<String, scratch::Block>, v: &[serde_json::Value]) -> PointTowardsOption {
    let id = v[1].as_str().expect("expected towards input option to point to a block");
    let block = &blocks[id];
    assert!(block.opcode == "motion_pointtowards_menu");

    match block.fields["TOWARDS"].0.as_str().unwrap() {
        "_mouse_" => PointTowardsOption::MouseCursor,
        s => PointTowardsOption::Actor(s.to_owned())
    }
}

impl std::str::FromStr for RotationStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all around" => Ok(RotationStyle::AllAround),
            "left-right" => Ok(RotationStyle::LeftRight),
            "don't rotate" => Ok(RotationStyle::DontRotate),
            _ => return Err(())
        }
    }
}

fn parse_rotation_style_option(_: &HashMap<String, scratch::Block>, v: &(serde_json::Value, Option<serde_json::Value>)) -> RotationStyle {
    v.0.as_str().unwrap().parse().expect("rotation style field attribute to be well formed")
}

fn parse_variable_option(_: &HashMap<String, scratch::Block>, v: &(serde_json::Value, Option<serde_json::Value>)) -> Variable {
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
    macro_rules! binop {
        ($outblock:ident, $inputname:literal) => {
            {
                let lhs = parse_value(blocks, &block.inputs[concat!($inputname, "1")]);
                let rhs = parse_value(blocks, &block.inputs[concat!($inputname, "2")]);
                Operator::$outblock { lhs, rhs }.into()
            }
        };
    }

    // pretty cursed macro to automate `parse_value` and `parse_*` calls for every input and field of a block. works pretty well though.
    // here is an example:
    //   normal_block!(Control::Repeat; times => "TIMES"; branch => parse_sequence_from_id_or_empty(inputs "SUBSTACK")),
    // was:
    // {
    //     let times = parse_value(blocks, &block.inputs["TIMES"]);
    //     let branch = parse_sequence_from_id_or_empty(blocks, &block.inputs["SUBSTACK"]);
    //     Control::Repeat { times, branch }.into()
    // }

    // first branch is for inputs and optional fields
    // second is for no inputs and fields
    // third is for macros with no fields (avoids ending semicolon)
    macro_rules! normal_block {
        ($kind:path; $($value:ident => $valuename:literal),+; $($option:ident => $func:ident($valueoroption:ident $optionname:literal)),*) => {
            {
                $kind {
                    $($value: parse_value(blocks, &block.inputs[$valuename])),+,
                    $($option: $func(blocks, &block.$valueoroption[$optionname])),*
                }.into()
            }
        };
        ($kind:path; ; $($option:ident => $func:ident($valueoroption:ident $optionname:literal)),+) => {
            {
                $kind {
                    $($option: $func(blocks, &block.$valueoroption[$optionname])),+
                }.into()
            }
        };
        ($kind:path; $($value:ident => $valuename:literal),+) => {
            {
                $kind {
                    $($value: parse_value(blocks, &block.inputs[$valuename])),+
                }.into()
            }
        };
    }

    match block.opcode.as_str() {
        "event_whenflagclicked" => Event::WhenFlagClicked.into(),
        "motion_movesteps" => normal_block!(Motion::MoveSteps; steps => "STEPS"),
        "motion_turnright" => normal_block!(Motion::TurnRight; degrees => "DEGREES"),
        "motion_turnleft" => normal_block!(Motion::TurnLeft; degrees => "DEGREES"),
        "motion_goto" => normal_block!(Motion::Goto;; dest => parse_goto_option(inputs "TO")),
        "motion_gotoxy" => normal_block!(Motion::GotoXY; x => "X", y => "Y"),
        // glide has the same parameters as goto
        "motion_glideto" => normal_block!(Motion::Glide; secs => "SECS"; dest => parse_goto_option(inputs "TO")),
        "motion_glidesecstoxy" => normal_block!(Motion::GlideXY; secs => "SECS", x => "X", y => "Y"),
        "motion_pointindirection" => normal_block!(Motion::PointInDrection; degrees => "DIRECTION"),
        "motion_pointtowards" => normal_block!(Motion::PointTowards;; towards => parse_towards_option(inputs "TOWARDS")),
        "motion_changexby" => normal_block!(Motion::ChangeX; by => "DX"),
        "motion_setx" => normal_block!(Motion::SetX; to => "X"),
        "motion_changeyby" => normal_block!(Motion::ChangeX; by => "DY"),
        "motion_sety" => normal_block!(Motion::SetX; to => "Y"),
        "motion_ifonedgebounce" => Motion::IfOnEdgeBounce.into(),
        "motion_setrotationstyle" => normal_block!(Motion::SetRotationStyle;; style => parse_rotation_style_option(fields "STYLE")),
        "motion_xposition" => Motion::XPosition.into(),
        "motion_yposition" => Motion::XPosition.into(),
        "motion_direction" => Motion::Direction.into(),
        "looks_sayforsecs" => normal_block!(Looks::SayForSecs; message => "MESSAGE", secs => "SECS"),
        "control_wait" => normal_block!(Control::Wait; duration => "DURATION"),
        "control_repeat" => normal_block! { Control::Repeat;
            times => "TIMES";
            branch => parse_sequence_from_id_or_empty(inputs "SUBSTACK")
        },
        "control_if" => normal_block! { Control::IfCondition;
            condition => "CONDITION";
            branch => parse_sequence_from_id_or_empty(inputs "SUBSTACK")
        },
        "control_create_clone_of" => normal_block!(Control::CreateCloneOf;; actor => parse_clone_option(inputs "CLONE_OPTION")),
        "data_setvariableto" => normal_block!(Data::SetVariableTo; value => "VALUE"; var => parse_variable_option(fields "VARIABLE")),
        "operator_add" => binop!(Add, "NUM"),
        "operator_subtract" => binop!(Sub, "NUM"),
        "operator_multiply" => binop!(Mul, "NUM"),
        "operator_divide" => binop!(Div, "NUM"),
        "operator_gt" => binop!(GreaterThan, "OPERAND"),
        "operator_lt" => binop!(LesserThan, "OPERAND"),
        "operator_equals" => binop!(Equals, "OPERAND"),
        "operator_and" => binop!(And, "OPERAND"),
        "operator_or" => binop!(Or, "OPERAND"),
        "operator_not" => normal_block!(Operator::Not; operand => "OPERAND"),
        opcode => todo!("unimplemented block: {opcode}"),
    }
}

/// parse a sequence if the given value has an ID, otherwise returns an empty sequence
fn parse_sequence_from_id_or_empty(blocks: &HashMap<String, scratch::Block>, input: &[serde_json::Value]) -> Sequence {
    if let Some(id) = input[1].as_str() {
        parse_sequence(blocks, &blocks[id])
    } else {
        Sequence(vec![])
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
/// 
/// variables on stage targets are always global
fn sanitize_varnames(is_stage: bool, vars: HashMap<String, scratch::Variable>, globals: &mut VarMap, global_hashset: &mut HashSet<String>) -> VarMap {
    let mut out = HashMap::new();
    let mut cidents = HashSet::new();

    for (id, var) in vars {
        let mut cident = var.0.replace(|x: char| !x.is_ascii_alphanumeric(), "_");

        if is_stage || var.2.is_some_and(|global| global) {
            while global_hashset.contains(&cident) {
                cident.push('_');
            }
            global_hashset.insert(cident.clone());
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

/// Checks if a block is allowed to start a sequence
/// This used to remove "floating blocks" that cannot start
fn is_block_real_toplevel(opcode: &str) -> bool {
    matches!(
        opcode,
        "event_whenflagclicked" |
        "event_whenkeypressed" |
        "event_whenthisspriteclicked" |
        "event_whenstageclicked" |
        "event_whenbackdropswitchesto" |
        "event_whengreaterthan" |
        "event_whenbroadcastreceived" |
        "control_start_as_clone"
    )
}

fn parse_target(mut target: scratch::Target, globals: &mut VarMap, global_hashset: &mut HashSet<String>) -> Target {
    let mut sequences = vec![];

    for (_, block) in &target.blocks {
        if block.topLevel {
            if !is_block_real_toplevel(&block.opcode) { continue }

            sequences.push(parse_sequence(&target.blocks, &block));
        }
    }

    let files: Vec<_> = std::fs::read_dir("project").unwrap().collect::<Result<_, _>>().unwrap();
    for costume in &mut target.costumes {
        if costume.md5ext.is_none() {
            let file = files.iter().find(|f| f.file_name().to_str().unwrap().starts_with(&costume.assetId)).expect("expected asset to be linked to a valid file");
            costume.md5ext = Some(file.file_name().to_str().unwrap().to_owned());
        }
    }

    let costumes = target.costumes.into_iter().map(|costume| {
        let filename = costume.md5ext.unwrap_or_else(|| format!("{}.{}", costume.assetId, costume.dataFormat));
        Costume {
            name: costume.name,
            filename,
            bitmap_resolution: costume.bitmapResolution,
            rotation_center_x: costume.rotationCenterX,
            rotation_center_y: costume.rotationCenterY
        }
    }).collect();

    let kind = if target.isStage {
        TargetKind::Stage { tempo: target.tempo.unwrap() }
    } else {
        TargetKind::Sprite {
            x: target.x.unwrap(),
            y: target.y.unwrap(),
            size: target.size.unwrap(),
            direction: target.direction.unwrap(),
            visible: target.visible.unwrap(),
            draggable: target.draggable.unwrap(),
            rotation_style: target.rotationStyle.unwrap().as_str().parse().expect("expected rotation style attribute to be well formed")
        }
    };

    Target {
        name: target.name,
        sequences,
        vars: sanitize_varnames(target.isStage, target.variables, globals, global_hashset),
        current_costume: target.currentCostume,
        costumes,
        sounds: target.sounds,
        kind
    }
}

pub fn parse(targets: Vec<scratch::Target>) -> (Vec<Target>, VarMap) {
    let mut globals = VarMap::new();
    let mut global_hashset = HashSet::new();

    let targets = targets.into_iter().map(|t| parse_target(t, &mut globals, &mut global_hashset)).collect();
    (targets, globals)
}
