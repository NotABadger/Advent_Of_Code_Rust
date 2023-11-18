#[derive(Clone, Copy, Debug)]
pub struct Elf
{
    pub number: u32,
}

impl Elf {
    pub fn new(number: u32) -> Self
    {
        Self { number }
    }
}

impl Default for Elf{
    fn default() -> Self {
        Self { number: 0 }
    }
}