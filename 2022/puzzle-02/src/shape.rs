use std::hash::Hash;

enum Shape {
    Rock,
    Paper,
    Scissor
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
        if (self == Self::Rock)
    }
}
impl Eq for Shape { }
