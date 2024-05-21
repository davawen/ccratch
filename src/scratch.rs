use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Vec<Value>>,
    pub fields: HashMap<String, (Value, Option<Value>)>,
    pub shadow: bool,
    pub topLevel: bool,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub comment: Option<String>,
    #[serde(default)]
    pub mutation: Value,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Costume {
    pub assetId: String,
    pub name: String,
    /// The name of the asset file
    /// If not present, use `assetId` with `dataFormat`
    #[serde(default)]
    pub md5ext: Option<String>,
    pub dataFormat: String,
    #[serde(default)]
    pub bitmapResolution: i32,
    pub rotationCenterX: i32,
    pub rotationCenterY: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Sound {
    pub assetId: String,
    pub name: String,
    /// The name of the asset file
    pub md5ext: String,
    pub dataFormat: String,
    pub rate: u64,
    pub sampleCount: u64,
}

#[derive(Debug, Deserialize)]
pub struct Variable(pub String, pub Value, #[serde(default)] pub Option<bool>);

#[derive(Debug, Deserialize)]
pub struct List(String, Vec<i32>);

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Target {
    pub isStage: bool,
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, Value>,
    pub blocks: HashMap<String, Block>,
    pub comments: HashMap<String, Value>,
    pub currentCostume: usize,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub layerOrder: i32,
    pub volume: u32,
    /// Stage specific
    #[serde(default)]
    pub tempo: Option<u32>,
    /// Stage specific
    #[serde(default)]
    pub videoState: Option<String>,
    /// Stage specific
    #[serde(default)]
    pub videoTransparency: Option<u32>,
    /// Stage specific
    #[serde(default)]
    pub textToSpeechLanguage: Option<String>,
    /// Sprite specific
    #[serde(default)]
    pub visible: Option<bool>,
    /// Sprite specific
    #[serde(default)]
    pub x: Option<f32>,
    /// Sprite specific
    #[serde(default)]
    pub y: Option<f32>,
    /// Sprite specific
    #[serde(default)]
    pub size: Option<f32>,
    /// Sprite specific
    #[serde(default)]
    pub direction: Option<f32>,
    /// Sprite specific
    #[serde(default)]
    pub draggable: Option<bool>,
    /// Sprite specific
    #[serde(default)]
    pub rotationStyle: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub targets: Vec<Target>,
    pub monitors: Value,
    pub extensions: Value,
    pub meta: Value,
}
