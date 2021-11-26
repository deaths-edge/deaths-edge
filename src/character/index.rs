#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterIndex(usize);

impl From<usize> for CharacterIndex {
    fn from(val: usize) -> Self {
        Self(val)
    }
}
