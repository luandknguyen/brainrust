/// Cells
#[derive(Clone, Debug)]
pub struct Cells(pub Vec<u8>);

impl Cells {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        if index < self.0.len() {
            Some(&mut self.0[index])
        } else {
            None
        }
    }

    pub fn get_mut_expand(&mut self, index: usize) -> Option<&mut u8> {
        if index < self.0.len() {
            Some(&mut self.0[index])
        } else {
            self.0.resize(index + 1, 0);
            Some(&mut self.0[index])
        }
    }
}
