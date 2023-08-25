use std::collections::HashMap;
use std::ops::Deref;


use crate::city::City;
use crate::route::Route;

#[derive(Debug)]
pub struct Country
{
    pub cities: HashMap<String, City>,
    pub routes: Vec<Route>,
}

impl Default for Country{
    fn default() -> Self {
        Self { cities: HashMap::new(), routes: Vec::new() }
    }
}

impl Country{
    pub fn is_city_known(&self, city_name: &str) -> bool
    {
        return self.cities.iter().any(|c| c.0.eq(city_name))
    }

    pub fn is_city_visited(&self, city_name: &str) -> bool
    {
        match self.cities.get(city_name)
        {
            Some(city) => return city.get_visited(),
            None => panic!("Searched for non-existing city"),
        }
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

    pub fn get_citys_visit_count(&self, city_name: &str) -> u32
    {
        match self.cities.get(city_name)
        {
            Some(city) => return city.get_visit_count(),
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
        let mut possible_routes: Vec<&Route> = Vec::new();
        for route in filtered_routes
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

    pub fn get_not_traveled_route_from_city(&self, city_name: &str) -> Option<Vec<Route>>
    {
        let filtered_routes = self.routes.iter().filter(|&r|r.connects_city_str(city_name) == true);
        let mut possible_routes: Vec<Route> = Vec::new();
        for route in filtered_routes
        {
            
            if route.connecting_cities.0 == city_name
            {
                if !self.is_city_visited(&route.connecting_cities.1)
                {
                    possible_routes.push(route.clone());
                } 
            }
            else 
            {
                if !self.is_city_visited(&route.connecting_cities.0)
                {
                    possible_routes.push(route.clone());
                }
            }
        }
        if possible_routes.is_empty()
        {
            return None;
        }
        Some(possible_routes)
    }

    pub fn get_roads_sorted_shortest_distance(&self) -> Vec<Route>
    {
        let mut ret_val: Vec<Route> = self.routes.clone();
        ret_val.sort_by(|r1, r2| r1.get_distance().cmp(&r2.get_distance()));
        ret_val
    }

    pub fn get_city_province(&self, city_name: &str) -> Option<u32>
    {
        match self.cities.get(city_name)
        {
            Some(city) => city.get_province(),
            None => {
                println!("Tried to get_province on non exisiting city: {:?}", city_name);
                panic!();
            }
        }
    }

    pub fn set_city_province(&self, city_name: &str, province: u32)
    {
        match self.cities.get(city_name)
        {
            Some(city) => city.set_province(province),
            None => {
                println!("Tried to set_province on exisiting city: {:?}", city_name);
                panic!();
            }
        }
    }

    pub fn merge_two_provinces(&self, province_1: u32, province_2: u32)
    {
        let itt = self.cities.iter().filter(|city| city.1.get_province() == Some(province_2));
        for city in itt
        {
            city.1.set_province(province_1);
        }
    }

}