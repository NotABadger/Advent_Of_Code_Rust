use crate::ingredient::{Ingredient, self};
use crate::recipe::{Recipe, Property};


pub fn create_coockie_recipy_1(ingredients: &Vec<Ingredient>) -> Recipe
{
    //fill recepy with equal parts of ingredients, and the tweak to see how far we can get.
    let mut cookie_recipy: RecipeCreator = RecipeCreator{working_recipe: Recipe::default()};
    for ingr in ingredients
    {
        cookie_recipy.working_recipe.add_possible_ingredient(ingr);
    }
    cookie_recipy.working_recipe.devide_ingredients_equally();
    println!("Values after adding all ingredients:");
    cookie_recipy.working_recipe.calculate_score(true);

    //now start tweaking
    cookie_recipy.calibrate_for_highest_score();
    return cookie_recipy.working_recipe;
}

struct RecipeCreator
{
    pub working_recipe: Recipe,
}

impl RecipeCreator
{

    fn calibrate_for_highest_score(&mut self)
    {
        let mut improved_score: bool = true;
        while improved_score 
        {
            improved_score = false;
            let amount_of_ingredients = self.working_recipe.available_ingredients.len();
            for ingr in 0..amount_of_ingredients
            {
                let ingredient: String = self.working_recipe.available_ingredients.get(ingr).expect("used len for max/min index").get_name();
                improved_score |= self.calibrate_ingredient(&ingredient);
            }
            //Loop through all possible ingredients
            //Check if adding ingredient instead of other will increase score
            //This checking will happen per ingredient replacing each other type of ingredient
            //If changing improved the score we will keep tweaking
            //After a while the max score has been found
        }
    }

    fn calibrate_ingredient(&mut self, ingredient: &str) -> bool
    {
        let mut improved_with_ingredient = false;
        let mut itt = self.working_recipe.available_ingredients.iter();
        for ingr in itt
        {
            let ingredient_to_remove = 
            if ingr.get_name() == ingredient
            {//can't replace ingredient with own ingredient
                continue;
            }

            let mut replacing_this_recipe_worked: bool = true;
            let mut old_score = self.working_recipe.calculate_score(false);
            while replacing_this_recipe_worked
            {
                self.replace_ingredient(ingredient, &ingr.get_name());
                let new_score = self.working_recipe.calculate_score(false);
                if old_score < new_score
                { //replacing improved score
                    self.replace_ingredient(&ingr.get_name(), ingredient); //undo
                    replacing_this_recipe_worked = false;
                }
                else 
                {
                    old_score = new_score;
                    improved_with_ingredient = true;
                }
            }
        }
        return improved_with_ingredient;
    }

    fn replace_ingredient(&mut self, ingredient_to_add: &str, ingredient_to_remove: &str)
    {
        todo!();
    }
}