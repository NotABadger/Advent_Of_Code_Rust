use std::ops::Index;

use crate::ingredient::Ingredient;


#[derive(Debug)]
pub struct Recipy
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

impl Recipy
{
    pub fn lowest_property(&self) -> Option<Property>
    {
        let mut properties: [i32; 5] = [0; 5];
        for ingr in &self.ingredients
        {
            properties[0] += ingr.get_capacity();
            properties[1] += ingr.get_durability();
            properties[2] += ingr.get_flavor();
            properties[3] += ingr.get_texture();
            properties[4] += ingr.get_calories();
        }
        //TODO 
        if  properties[0] <= properties[1] &&
            properties[0] <= properties[2] &&
            properties[0] <= properties[3]// &&
            //properties[0] <= properties[4] )
            {
                return Some(Property::Capacity);
            }
        if  properties[1] <= properties[0] &&
            properties[1] <= properties[2] &&
            properties[1] <= properties[3]// &&
            //properties[0] <= properties[4] )
            {
                return Some(Property::Durability);
            }
        if  properties[2] <= properties[1] &&
            properties[2] <= properties[0] &&
            properties[2] <= properties[3]// &&
            //properties[0] <= properties[4] )
            {
                return Some(Property::Flavor);
            }
        if  properties[3] <= properties[1] &&
            properties[3] <= properties[2] &&
            properties[3] <= properties[0]// &&
            //properties[0] <= properties[4] )
            {
                return Some(Property::Texture);
            }
        if  properties[4] <= properties[1] &&
            properties[4] <= properties[2] &&
            properties[4] <= properties[3]// &&
            //properties[4] <= properties[0] )
            {
                return Some(Property::Calories);
            }

            return None;
    }

}

impl Default for Recipy
{
    fn default() -> Self
    {
        Self{ingredients: Vec::with_capacity(100)}
    }
}