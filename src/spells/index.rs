pub struct SpellIndex(usize);

impl From<usize> for SpellIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
