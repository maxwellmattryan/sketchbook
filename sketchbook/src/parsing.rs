use crate::error::{SketchbookError, SketchbookResult};

use regex::Regex;

pub(crate) fn try_sketch_name_from_str(arg: &str) -> SketchbookResult<String> {
    let re = Regex::new(r"[^ -_.a-zA-Z0-9]").unwrap();
    if re.is_match(arg) {
        Err(SketchbookError::InvalidSketchName(arg.to_string()))
    } else {
        Ok(arg.to_string())
    }
}
