use std::collections::HashMap;

use crate::ingredient::Ingredient;

const MAX_AMOUNT_OF_INGREDIENTS: u32 = 100;
#[derive(Debug)]
pub struct Recipe
{
    pub available_ingredients: Vec<Ingredient>,
    pub amount_of_ingredient: HashMap<String, u32>
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
    pub fn add_possible_ingredient(&mut self, ingredient: &Ingredient)
    {
        self.available_ingredients.push(ingredient.clone());
        self.amount_of_ingredient.insert(ingredient.get_name(), 0);
    }

    pub fn devide_ingredients_equally(&mut self)
    {
        let amount_per_ingredient: u32 = MAX_AMOUNT_OF_INGREDIENTS / self.available_ingredients.len() as u32;
        for ingr in &self.available_ingredients
        {
            let ingredient_quantity = self.amount_of_ingredient.get_mut(&ingr.get_name()).unwrap();
            *ingredient_quantity = amount_per_ingredient;
        }
    }

    pub fn calculate_score(&self, print: bool) -> u32
    {
        let mut capacity_score: i32 = 0;
        let mut durability_score: i32 = 0;
        let mut flavor_score: i32 = 0;
        let mut texture_score: i32 = 0;
        let mut calories_score: i32 = 0;

        for type_ingredient in &self.amount_of_ingredient
        {//retrieve type of ingredient, add values for the amount of times in the recepy
            let ingr: &Ingredient = self.available_ingredients.iter().find(|&i| &i.get_name() == type_ingredient.0).unwrap();
            for index in 0..*type_ingredient.1
            {
                capacity_score += ingr.get_capacity();
                durability_score += ingr.get_durability();
                flavor_score += ingr.get_flavor();
                texture_score += ingr.get_texture();
                calories_score += ingr.get_calories();
            }
        }

        if print
        {
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
        let total_score: u32 = capacity_score as u32 * durability_score as u32 * flavor_score as u32 * texture_score as u32; // * calories_score as u32;
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
        Self{available_ingredients: Vec::new(), amount_of_ingredient: HashMap::new()}
    }
}