#![allow(dead_code)]

pub use crate::templates::blank::main as blank_main;

#[derive(Debug)]
pub enum Sketch {
    Blank,
}

pub type SketchEntrypoint = fn() -> ();

impl Sketch {
    pub fn to_entrypoint(&self) -> SketchEntrypoint {
        match self {
            Sketch::Blank => blank_main,
        }
    }
}

impl std::fmt::Display for Sketch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Sketch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blank" | "Blank" => Ok(Sketch::Blank),
            _ => Err(format!("\"{}\" is not a valid sketch", s)),
        }
    }
}
