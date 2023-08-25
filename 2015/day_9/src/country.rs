use std::collections::HashMap;
use std::ops::Deref;


use crate::city::City;
use crate::route::Route;

#[derive(Debug)]
pub struct Country<'a>
{
    pub cities: HashMap<String, City<'a>>,
    pub routes: Vec<Route>,
}

impl Default for Country<'_>{
    fn default() -> Self {
        Self { cities: HashMap::new(), routes: Vec::new() }
    }
}

impl Country<'_>{
    pub fn is_city_known(&self, city_name: &str) -> bool
    {
        return self.cities.iter().any(|c| c.0.eq(city_name))
    }

    pub fn is_city_visited(&self, city_name: &str) -> bool
    {
        return  self.cities.get(city_name).expect("Searched for a non-existing city").get_visited();
    }

    pub fn is_route_known(&self, city1_name: &str, city2_name: &str) -> bool
    {
        return self.routes.iter().any(|r| r.connects_city_str(city1_name) && r.connects_city_str(city2_name));
    }

    pub fn visit_city(&self, city_name: &str)
    {
        match self.cities.get(city_name)
        {
            Some(city) => city.visit_city(),
            None => {
                println!("Tried to visit non exisiting city: {:?}", city_name);
                panic!();
            }
        }
    }

    pub fn reset_all_travel(&self)
    {
        for city in &self.cities
        {
            city.1.reset_visit();
        }
    }

    pub fn get_amount_visited_countries(&self) -> u32
    {
        let mut visited: u32 = 0;
        for city in &self.cities
        {
            if city.1.get_visited()
            {
                visited += 1;
            }
        }
        visited
    }

    pub fn get_shortest_not_traveled_route(&self, city_name: &str) -> Option<Route>
    {
        let filtered_routes = self.routes.iter().filter(|r|r.connects_city_str(city_name) == true);
        let filtered_routes_vec : Vec<&Route> = filtered_routes.collect();

        let mut possible_routes: Vec<&Route> = Vec::new();
        for route in filtered_routes_vec
        {
            
            if route.connecting_cities.0 == city_name
            {
                if !self.is_city_visited(&route.connecting_cities.1)
                {
                    possible_routes.push(route);
                } 
            }
            else 
            {
                if !self.is_city_visited(&route.connecting_cities.0)
                {
                    possible_routes.push(route);
                }
            }
        }

        let temp = possible_routes.iter().min_by(|r1, r2| r1.get_distance().cmp(&r2.get_distance()));
        match temp{
            Some(val) => return Some(val.deref().deref().clone()),
            None => return None,
        }
    }

}