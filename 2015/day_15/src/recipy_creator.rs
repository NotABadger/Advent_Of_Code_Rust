use crate::ingredient::Ingredient;
use crate::recipy::{Recipy, Property};

pub fn create_coockie_recipy_1(ingredients: &Vec<Ingredient>) -> Recipy
{
    //See which ingredient adds the most value/fullness. add as much as you can.
    //See which value's go negative, add components to compensate
    let mut cookie_recipy: Recipy = Recipy::default();
    let mut ordered_by_fullness = ingredients.clone();
    ordered_by_fullness.sort_by(|ingr1, ingr2| ingr1.get_fullness().cmp(&ingr2.get_fullness()));
    while cookie_recipy.ingredients.len() < 99
    {
        match cookie_recipy.lowest_property()
        {
            None => cookie_recipy.ingredients.push(ordered_by_fullness.get(0).expect("Hand-programmed the values myself...").clone()),
            Some(val) => {
                cookie_recipy.ingredients.push(
                    get_supporting_ingredient(val, &ordered_by_fullness)
                    .clone());
            },
        }
    }
    
    println!("Highest density ingredient: {}", ordered_by_fullness.get(0).expect("Hand-programmed the values myself...").get_name());
    return cookie_recipy;
}

// fn calculate_score(recipy: &Vec<Ingredient>)
// {
//     let mut capacity_score: i32 = 0;
//     let mut durability_score: i32 = 0;
//     let mut flavor_score: i32 = 0;
//     let mut texture_score: i32 = 0;
//     let mut calories_score: i32 = 0;

//     for ingredient in recipy
//     {
//         capacity_score += ingredient.get_capacity();
//         durability_score += ingredient.get_durability();
//         flavor_score += ingredient.get_flavor();
//         texture_score += ingredient.get_texture();
//         calories_score += ingredient.get_calories();
//     }
// }

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