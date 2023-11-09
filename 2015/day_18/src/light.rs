
#[derive(Clone, Debug)]
pub struct Light{
    pub state:bool,
}

impl Default for Light {
    fn default() -> Self {
        Self { state: false }
    }
}