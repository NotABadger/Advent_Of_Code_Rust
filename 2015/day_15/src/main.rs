mod ingredient;
mod recipy_creator;
mod recipy;

use ingredient::Ingredient;
use recipy::Recipy;
use recipy_creator::create_coockie_recipy_1;

fn main() {

    let cookie_recipy: Recipy; // = Vec::with_capacity(100);
    let mut available_ingredients: Vec<Ingredient> = Vec::new();
    available_ingredients.push(Ingredient::new("Frosting", 4, -2, 0, 0, 5));
    available_ingredients.push(Ingredient::new("Candy", 0, 5, -1, 0, 8));
    available_ingredients.push(Ingredient::new("Butterscotch", -1, 0, 5, 0, 6));
    available_ingredients.push(Ingredient::new("Sugar", 0, 0, -2, 2, 1));
    

    //Logic on balancing ingredients\\
    cookie_recipy = create_coockie_recipy_1(&available_ingredients);

    calculate_score(&cookie_recipy);
}

fn calculate_score(recipy: &Recipy)
{
    let mut capacity_score: i32 = 0;
    let mut durability_score: i32 = 0;
    let mut flavor_score: i32 = 0;
    let mut texture_score: i32 = 0;
    let mut calories_score: i32 = 0;

    for ingredient in &recipy.ingredients
    {
        capacity_score += ingredient.get_capacity();
        durability_score += ingredient.get_durability();
        flavor_score += ingredient.get_flavor();
        texture_score += ingredient.get_texture();
        calories_score += ingredient.get_calories();
    }

    println!("Mixed all ingredients!");
    println!("Total capacity score: {}", capacity_score);
    println!("Total durability score: {}", durability_score);
    println!("Total flavor score: {}", flavor_score);
    println!("Total texture score: {}", texture_score);
    println!("Total calories score: {}", calories_score);

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
    println!("Total overall score is: {}!", total_score);
}