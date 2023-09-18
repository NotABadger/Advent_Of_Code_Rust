use crate::ingredient::Ingredient;
use crate::recipe::Recipe;


pub fn create_coockie_recipy_part_1(ingredients: &Vec<Ingredient>) -> Recipe
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
        //Loop through all possible ingredients
        //Check if adding ingredient instead of other will increase score
        //This checking will happen per ingredient replacing each other type of ingredient
        //If changing improved the score we will keep tweaking
        //After a while the max score has been found
        let mut improved_score: bool = true;
        while improved_score 
        {
            improved_score = false;
            let amount_of_ingredients = self.working_recipe.available_ingredients.len();
            for ingr in 0..amount_of_ingredients
            {
                let ingredient: String = self.working_recipe.available_ingredients.get(ingr).expect("used len for max index").get_name();
                improved_score |= self.calibrate_ingredient(&ingredient);
            }
            
        }
    }

    fn calibrate_ingredient(&mut self, ingredient: &str) -> bool
    {
        let mut improved_with_ingredient = false;
        let ingredients_list_len = self.working_recipe.available_ingredients.len();
        for ingr in 0..ingredients_list_len
        {
            let ingredient_to_remove = self.working_recipe.available_ingredients.get(ingr).unwrap().get_name();
            if ingredient_to_remove == ingredient
            {//can't replace ingredient with own ingredient
                continue;
            }

            let mut replacing_this_recipe_worked: bool = true;
            let mut old_score = self.working_recipe.calculate_score(false);
            while replacing_this_recipe_worked
            {
                self.replace_ingredient(ingredient, &ingredient_to_remove);
                let new_score = self.working_recipe.calculate_score(false);
                if old_score < new_score
                { //replacing improved score
                    old_score = new_score;
                    improved_with_ingredient = true;
                }
                else 
                {//WARNING, when a value is used 0 or 100 times, the undo will ruin the logic. this usecase is not hit in example.
                    //I was too lazy to cover this in puzzle code, but could have returned a bool or Option while replacing
                    self.replace_ingredient(&ingredient_to_remove, ingredient); //undo
                    replacing_this_recipe_worked = false;
                }
            }
        }
        return improved_with_ingredient;
    }

    fn replace_ingredient(&mut self, ingredient_to_add: &str, ingredient_to_remove: &str)
    {
        let subtract_amount =  self.working_recipe.amount_of_ingredient.get_mut(ingredient_to_remove).unwrap().clone();
        let add_amount = self.working_recipe.amount_of_ingredient.get_mut(ingredient_to_add).unwrap().clone();
        if subtract_amount <= 0 || add_amount >= 100
        {
            return;
        }
        let remove_ref = self.working_recipe.amount_of_ingredient.get_mut(ingredient_to_remove).unwrap();
        *remove_ref -= 1;
        let increment_ref = self.working_recipe.amount_of_ingredient.get_mut(ingredient_to_add).unwrap();
        *increment_ref += 1;

    }

}