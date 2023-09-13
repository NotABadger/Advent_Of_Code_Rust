mod ingredient;
mod recipe_creator;
mod recipe;

use ingredient::Ingredient;
use recipe::Recipe;
use recipe_creator::create_coockie_recipy_1;

fn main() {

    let cookie_recipe: Recipe; // = Vec::with_capacity(100);
    let mut available_ingredients: Vec<Ingredient> = Vec::new();
    available_ingredients.push(Ingredient::new("Frosting", 4, -2, 0, 0, 5));
    available_ingredients.push(Ingredient::new("Candy", 0, 5, -1, 0, 8));
    available_ingredients.push(Ingredient::new("Butterscotch", -1, 0, 5, 0, 6));
    available_ingredients.push(Ingredient::new("Sugar", 0, 0, -2, 2, 1));
    

    //Logic on balancing ingredients\\
    cookie_recipe = create_coockie_recipy_1(&available_ingredients);

    cookie_recipe.calculate_score(true);
}

