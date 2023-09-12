

#[derive(Debug, Clone)]
pub struct Ingredient
{
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
    fullness: i32,
}

impl Ingredient {
    pub fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self
    {
        let fullness: i32 = capacity + durability + flavor + texture; // + calories
        Self{name: name.to_string(), capacity, durability, flavor, texture, calories, fullness}
    }

    pub fn get_name(&self) -> String
    {
        self.name.to_string()
    }
    pub fn get_capacity(&self) -> i32
    {
        self.capacity
    }
    pub fn get_durability(&self) -> i32
    {
        self.durability
    }
    pub fn get_flavor(&self) -> i32
    {
        self.flavor
    }
    pub fn get_texture(&self) -> i32
    {
        self.texture
    }
    pub fn get_calories(&self) -> i32
    {
        self.calories
    }
    pub fn get_fullness(&self) -> i32
    {
        self.fullness
    }


}