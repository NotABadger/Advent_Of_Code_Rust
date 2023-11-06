mod ingredient;
mod recipe_creator_part_1;
mod recipe_creator_part_2;
mod recipe;

use ingredient::Ingredient;
use recipe::Recipe;
use recipe_creator_part_1::create_coockie_recipy_part_1;
use recipe_creator_part_2::create_coockie_recipy_part_2;

fn main() {

    let cookie_recipe: Recipe;
    let max_cal_cookie_recipe: Recipe;
    let mut available_ingredients: Vec<Ingredient> = Vec::new();
    available_ingredients.push(Ingredient::new("Frosting", 4, -2, 0, 0, 5));
    available_ingredients.push(Ingredient::new("Candy", 0, 5, -1, 0, 8));
    available_ingredients.push(Ingredient::new("Butterscotch", -1, 0, 5, 0, 6));
    available_ingredients.push(Ingredient::new("Sugar", 0, 0, -2, 2, 1));
    

    //Logic on balancing ingredients\\
    cookie_recipe = create_coockie_recipy_part_1(&available_ingredients);
    println!("The highest score:");
    cookie_recipe.calculate_score(true, false);
    for ind in cookie_recipe.amount_of_ingredient
    {
        println!("Recepy containes {} of {}", ind.1, ind.0);
    }
    
    max_cal_cookie_recipe = create_coockie_recipy_part_2(&available_ingredients);
    println!("The highest score for 500 kcal:");
    max_cal_cookie_recipe.calculate_score(true, false);
    for ind in max_cal_cookie_recipe.amount_of_ingredient
    {
        println!("Recepy containes {} of {}", ind.1, ind.0);
    }

}

