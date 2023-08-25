
#[derive(Debug, Clone)]
pub struct Province
{
    id: u32,
}

impl Province
{
    pub fn new(id : u32) -> Self
    {
        Self{id}
    }

    pub fn get_id(&self) -> u32
    {
        self.id
    }
}