use crate::{country::Country, route::Route};
use std::rc::Rc;

/* This algoritm will brute force with all roads per city, and all cities as a starting point
    Every city will request the all roads that goes to a city that hasn't been visited.
    Recursion will stop when no all routes have been checked */

pub fn resolve_traveler_problem(country: Rc<Country>) -> Option<u32>
{
    let mut routes_taken: Vec<Route> = Vec::new();
    let mut cities_visited: Vec<String> = Vec::new();
    let mut shortest_path_all_cities: Option<u32> = None;
    /*grab shortest route from this city, travel it if other city is unvisited */

    for itt in country.cities.iter()
    {//take every city as starting point
        let mut roads_taken: Vec<Route> = Vec::new();
        let mut city_order: Vec<String> = Vec::new();
        let shortest_path_this_city: u32 = travel_all_roads(country.clone(), &itt.0, &mut roads_taken, &mut city_order).expect("would at least expect 1 city traveled");
        let amount_of_cities_connected: u32 = country.get_amount_visited_countries(); // start with current city

        //see if we have connected all cities
        if amount_of_cities_connected == country.cities.len() as u32
        {
            match shortest_path_all_cities
            {
                Some(value) => {
                    //if shortest_path_this_city < value
                    if shortest_path_this_city > value //part two
                    {
                        shortest_path_all_cities = Some(shortest_path_this_city);
                        routes_taken = roads_taken;
                        cities_visited = city_order;
                    }
                },
                None =>shortest_path_all_cities = Some(shortest_path_this_city),
            } 
        }
        country.reset_all_travel();
    }

    // println!("Visiting order: ");
    // println!("{:#?}", cities_visited);
    // println!("Routes taken: ");
    // println!("{:#?}", routes_taken);
    shortest_path_all_cities
}

pub fn travel_all_roads(country: Rc<Country>, city: &str, roads_taken: &mut Vec<Route>, cities_visited: &mut Vec<String>) -> Option<u32>
{
    country.visit_city(city);
    cities_visited.push(city.to_string()); //only admin for route info
    
    let routes = country.get_not_traveled_route_from_city(city);
    if routes.is_none()
    {
        let mut total_distance = 0;
        roads_taken.iter().for_each(|road| total_distance += road.get_distance());
        println!("End of road, total distance: {:?}",total_distance);
        println!("roads taken: {:?}",roads_taken);
        roads_taken.pop();
        cities_visited.pop();
        return None;
    }

    for road in routes.unwrap()//order of roads doesn't matter, all need to be tried
    {
        
        roads_taken.push(road.clone());
        let total_traveled_distance: u32 = road.get_distance();
        match travel_all_roads(country.clone(), &road.get_destination_city(city), roads_taken, cities_visited) 
        {
            Some(distance) => return Some(1),
            None => continue,
        } 
    }
    roads_taken.pop();
    cities_visited.pop();
    return Some(1);
}