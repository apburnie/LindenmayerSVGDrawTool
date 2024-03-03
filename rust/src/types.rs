#[derive(Debug)]
pub struct Memory {
    pub x: Vec<u128>,
    pub y: Vec<u128>,
    pub angle: Vec<u128>,
}

#[derive(Debug)]
pub struct StartPosition {
    pub x: u128,
    pub y: u128,
    pub angle: u128,
    pub memory: Memory,
}

#[derive(Debug)]
pub struct StringConvert<'a> {
    pub m: &'a str,
    pub l: &'a str,
    pub f: &'a str,
}

#[derive(Debug)]
pub struct Rules<'a> {
    pub recursions: u8,
    pub axiom: char,
    pub start_position: StartPosition,
    pub string_convert: StringConvert<'a>,
}

#[derive(Debug, Clone)]
pub struct CurrentPosition {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

#[derive(Debug, Clone)]
pub struct DrawPosition {
    pub current: CurrentPosition,
    pub memory: Vec<CurrentPosition>,
}
