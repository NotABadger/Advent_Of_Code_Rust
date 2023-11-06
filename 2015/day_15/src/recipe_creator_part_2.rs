use crate::Recipe;
use crate::recipe;
use crate::Ingredient;


//Since my skills in mathmatics are not good enough, 
// and this is just a puzzle so it's time to permutate all possible combinations!
pub fn create_coockie_recipy_part_2(ingredients: &Vec<Ingredient>) -> Recipe
{
    let mut cookie_recipe: RecipeCreator = RecipeCreator{working_recipe: Recipe::default()};
    let mut possible_recipies: Vec<Recipe> = Vec::new();
    for ingr in ingredients
    {
        cookie_recipe.working_recipe.add_possible_ingredient(ingr);
    }
    cookie_recipe.permutate_ingredient_possibilities(0, &mut possible_recipies);
    
    //Grab the highest value recipe from possible recipy list and return that one

    return possible_recipies.iter().max_by(|&a, &b| 
        a.calculate_score(false, true).cmp(&b.calculate_score(false, true))).expect("At lease some combo should be valid").clone();
}

#[derive(Debug, Clone)]
struct RecipeCreator
{
    pub working_recipe: Recipe,
}

impl RecipeCreator
{
    fn permutate_ingredient_possibilities(&mut self, ingredient_index: usize, possible_recipe_list: &mut Vec<Recipe>)
    {
        let last_ingredient: bool = ingredient_index == self.working_recipe.available_ingredients.len() -1;
        // Calculate all possible permutations
        let mut amount_already_used: u32 = 0;
        for ingredient in &self.working_recipe.amount_of_ingredient
        {
            // add amount per ingredient
            amount_already_used += ingredient.1;
        }
        // -> Get max amount we can add
        let mut amount_to_add = recipe::MAX_AMOUNT_OF_INGREDIENTS - amount_already_used;
        if amount_to_add == 0
        {// -> nothing to add
            return;
        }
        // -> Start with max ingredient.
        self.working_recipe.amount_of_ingredient.insert(
            self.working_recipe.available_ingredients.get(ingredient_index).expect("Should be here").get_name(), 
            amount_to_add);
        
        // -> check viable 500kcal recipe
        if self.working_recipe.calculate_score(false, true) > 0
        {
            possible_recipe_list.push(self.working_recipe.clone());
        }
        if last_ingredient 
        { //if last, remove ingredient and return
            self.working_recipe.amount_of_ingredient.remove(&self.working_recipe.available_ingredients.get(ingredient_index).expect("Should be here").get_name());
            return;
        }
        
        while amount_to_add > 0
        {
            self.permutate_ingredient_possibilities(ingredient_index +1, possible_recipe_list);
            //remove 1 from self 
            amount_to_add = amount_to_add - 1;
            let reference_to_amount = self.working_recipe.amount_of_ingredient.get_mut(
                &self.working_recipe.available_ingredients.get(ingredient_index).expect("Should be here").get_name())
                .expect("added this earler");
            *reference_to_amount = amount_to_add;
        }
      
        self.working_recipe.amount_of_ingredient.remove(&self.working_recipe.available_ingredients.get(ingredient_index).expect("Should be here").get_name());
        return;
    }
}