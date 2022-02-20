use sketches::{sketches::ten_print_main, templates::blank_main};

#[derive(Debug)]
pub enum Sketch {
    Blank,
    TenPrint,
}

pub type SketchEntrypoint = fn() -> ();

impl Sketch {
    pub fn to_entrypoint(&self) -> SketchEntrypoint {
        match self {
            Sketch::Blank => blank_main,
            Sketch::TenPrint => ten_print_main,
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
            "ten_print" | "Ten Print" => Ok(Sketch::TenPrint),
            _ => Err(format!("\"{}\" is not a valid sketch", s)),
        }
    }
}
