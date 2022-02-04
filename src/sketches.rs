#![allow(dead_code)]

mod empty;
pub use empty::main as empty_main;

mod empty_test;
pub use empty_test::main as empty_test_main;

#[derive(Debug)]
pub enum Sketch {
    Empty,
    EmptyTest,
}

pub type SketchEntrypoint = fn() -> ();

impl Sketch {
    pub fn to_entrypoint(&self) -> SketchEntrypoint {
        match self {
            Sketch::Empty => empty_main,
            Sketch::EmptyTest => empty_test_main,
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
            "empty" | "Empty" => Ok(Sketch::Empty),
            "empty_test" | "EmptyTest" => Ok(Sketch::EmptyTest),
            _ => Err(format!("\"{}\" is not a valid sketch", s)),
        }
    }
}
