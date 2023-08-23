use crate::city::City;


#[derive(Debug, Clone)]
pub struct Route
{
    pub connecting_cities:(String,String),
    distance: u32,
}

impl Route
{
    pub fn new(city_1: &str, city_2: &str, distance: u32) -> Self
    {
        Self{connecting_cities: (city_1.to_string(), city_2.to_string()), distance: distance}
    }

    pub fn get_distance(&self) -> u32
    {
        self.distance
    }

    pub fn connects_city_ref(&self, city: &City) -> bool
    {
        self.connects_city_str(&city.get_name())
    }

    pub fn connects_city_str(&self, city: &str) -> bool
    {
        if self.connecting_cities.0 == city || self.connecting_cities.1 == city
        {
            return true;
        }
        false
    }

    pub fn get_destination_city(&self, start_city: &str) -> String
    {
        if self.connecting_cities.0 == start_city
        {
            return self.connecting_cities.1.to_string();
        }
        return self.connecting_cities.0.to_string();
    }
}
