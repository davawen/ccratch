
mod scratch;
mod parser;
mod generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("./project/project.json")?;

    let project: scratch::Project = serde_json::from_str(&data)?;
    let targets = project.targets;
    let (targets, globals) = parser::parse(targets);

    let mut out = std::fs::File::create("output.c")?;
    generator::generate(&mut out, &targets, &globals)?;
    Ok(())
}
