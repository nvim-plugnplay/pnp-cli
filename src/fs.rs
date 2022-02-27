use std::path::Path;

pub struct Exists {
    pub path: bool,
    pub git: bool,
}

impl Exists {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path).exists(),
            git: Path::new(&format!("{path}/.git")).exists(),
        }
    }
}
