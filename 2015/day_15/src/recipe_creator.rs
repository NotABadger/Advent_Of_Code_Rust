use crate::ingredient::Ingredient;
use crate::recipe::{Recipe, Property};

pub fn create_coockie_recipy_1(ingredients: &Vec<Ingredient>) -> Recipe
{
    //fill recepy with equal parts of ingredients, and the tweak to see how far we can get.
    let mut cookie_recipy: Recipe = Recipe::default();

    let mut ordered_by_fullness = ingredients.clone();
    


    println!("Highest density ingredient: {}", ordered_by_fullness.get(0).expect("Hand-programmed the values myself...").get_name());
    return cookie_recipy;
}

fn get_supporting_ingredient(property_to_fix: Property, fullness_ordered_ingredient_list: &Vec<Ingredient>) -> &Ingredient 
{
    match property_to_fix { // original intention was to use find and compensate with the fullest alternative (see ful parameter in ingredient)
        Property::Capacity => {
            return fullness_ordered_ingredient_list.iter().max_by(|&ingr1, &ingr2| ingr1.get_capacity().cmp(&ingr2.get_capacity())).unwrap();
        }
        Property::Durability => {
            return fullness_ordered_ingredient_list.iter().max_by(|&ingr1, &ingr2| ingr1.get_durability().cmp(&ingr2.get_durability())).unwrap();
        },
        Property::Flavor => {
            return fullness_ordered_ingredient_list.iter().max_by(|&ingr1, &ingr2| ingr1.get_flavor().cmp(&ingr2.get_flavor())).unwrap();
        },
        Property::Texture => {
            return fullness_ordered_ingredient_list.iter().max_by(|&ingr1, &ingr2| ingr1.get_texture().cmp(&ingr2.get_texture())).unwrap();
        },
        Property::Calories => {
            return fullness_ordered_ingredient_list.iter().max_by(|&ingr1, &ingr2| ingr1.get_calories().cmp(&ingr2.get_calories())).unwrap();
        },
        _ => panic!("impossibruuuuu!!!"),
    }
}