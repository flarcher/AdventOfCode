use std::{hash::Hash, fmt::{Display}};

#[derive(Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissor
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let str = &self.to_string();
      f.write_str(str)
    }
}

impl Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash = match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        };
        state.write_i8(hash);
    }
}
impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Eq for Shape { }
