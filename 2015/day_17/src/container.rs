
#[derive(Debug, Clone, Copy)]
pub struct ContainerType{
    pub capacity: u32,
}


impl ContainerType{
    pub fn new(capacity: u32) -> Self
    {
        Self { capacity }
    }
}