
#[derive(PartialEq)]
pub struct Wire
{
    name : String,
    value : Option<u16>,
}

impl Wire
{
    pub fn new(name: &str) -> Self
    {
        Self{name: name.to_string(), value: None}
    }

    pub fn get_name(&self) -> String 
    {
        self.name.to_string()
    }

    pub fn set_value(&mut self, value: u16) 
    {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<u16> 
    {
        self.value
    }
    
}