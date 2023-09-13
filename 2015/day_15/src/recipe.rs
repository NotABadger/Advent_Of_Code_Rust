use crate::ingredient::Ingredient;

#[derive(Debug)]
pub struct Recipe
{
    pub ingredients: Vec<Ingredient>,
}

pub enum Property
{
    Capacity,
    Durability,
    Flavor,
    Texture,
    Calories,
}

impl Recipe
{
    pub fn calculate_score(&self, print: bool) -> i32
    {
        let mut capacity_score: i32 = 0;
        let mut durability_score: i32 = 0;
        let mut flavor_score: i32 = 0;
        let mut texture_score: i32 = 0;
        let mut calories_score: i32 = 0;

        for ingredient in &self.ingredients
        {
            capacity_score += ingredient.get_capacity();
            durability_score += ingredient.get_durability();
            flavor_score += ingredient.get_flavor();
            texture_score += ingredient.get_texture();
            calories_score += ingredient.get_calories();
        }

        if print
        {
            println!("Mixed all ingredients!");
            println!("Total capacity score: {}", capacity_score);
            println!("Total durability score: {}", durability_score);
            println!("Total flavor score: {}", flavor_score);
            println!("Total texture score: {}", texture_score);
            println!("Total calories score: {}", calories_score);
        }
        if capacity_score < 0
        {
            capacity_score = 0;
        }
        if durability_score < 0
        {
            durability_score = 0;
        }
        if flavor_score < 0
        {
            flavor_score = 0;
        }
        if texture_score < 0
        {
            texture_score = 0;
        }
        if calories_score < 0
        {
            calories_score = 0;
        }
        let total_score: i32 = capacity_score * durability_score * flavor_score * texture_score; // * calories_score;
        if print
        {
            println!("Total overall score is: {}!", total_score);
        }
        return total_score;
    }
}

impl Default for Recipe
{
    fn default() -> Self
    {
        Self{ingredients: Vec::with_capacity(100)}
    }
}