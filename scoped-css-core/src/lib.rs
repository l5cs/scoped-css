use std::ops::Index;

#[derive(Debug)]
pub struct ScopedStyles {
    pub classes: &'static [(&'static str, &'static str)],
}

impl Index<&'static str> for ScopedStyles {
    type Output = str;

    fn index(&self, index: &'static str) -> &Self::Output {
        self.classes
            .iter()
            .find_map(|(original, modified)| (*original == index).then_some(*modified))
            .unwrap_or_default()
    }
}
