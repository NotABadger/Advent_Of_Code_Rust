use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct City
{
    pub name: String,
    pub connections: HashMap<String, u32>,
}

impl City
{
    pub fn set_name(&mut self, name: &str)
    {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> String
    {
        self.name.to_string()
    }

    pub fn add_city_relation(&mut self, name : &str, distance: u32)
    {
        if self.connections.get(name).is_none()
        {
            self.connections.insert(name.to_string(), distance);
        }
    }
}