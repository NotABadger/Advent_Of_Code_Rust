use crate::{country::Country, route::Route};
use std::rc::Rc;


/* This algoritm will brute force with all roads per city, and all cities as a starting point
    Every city will request the all roads that goes to a city that hasn't been visited.
    Recursion will stop when no all routes have been checked */

pub fn resolve_traveler_problem(country: Rc<Country>)
{
    let mut routes_tested: u32 = 0;
    let mut longest_route: u32 = 0;
    let mut shortest_route: u32 = u32::MAX;


    for itt in country.cities.iter()
    {//take every city as starting point
        let mut roads_taken: Vec<Route> = Vec::new();       
        travel_all_roads(country.clone(), &itt.0, &mut roads_taken,&mut routes_tested, &mut shortest_route, &mut longest_route);
        country.reset_all_travel();
    }

    println!("amount of routes tested: {:?}", routes_tested);
    println!("Brute force found shortest route: {:?}", shortest_route);
    println!("Brute force found longest route: {:?}", longest_route);
}

pub fn travel_all_roads(country: Rc<Country>, city: &str, roads_taken: &mut Vec<Route>, routes_tested: &mut u32, shortest_road: &mut u32, longest_road: &mut u32)
{
    /*  Visit current city, and retrieve possible routes to not yet visited cities
        when no more cities are available, print result of route
        When roads are available, travel them ALL.*/
    country.visit_city(city);
    
    let routes = country.get_not_traveled_route_from_city(city);
    if routes.is_none()
    {
        let mut total_distance = 0;
        roads_taken.iter().for_each(|road| total_distance += road.get_distance());
        if total_distance > *longest_road
        {
            *longest_road = total_distance;
        }
        if total_distance < *shortest_road
        {
            *shortest_road = total_distance;
        }
        *routes_tested += 1;
        //println!("End of road, total distance: {:?}",total_distance);
        //println!("roads taken: {:?}",roads_taken);
        roads_taken.pop();
        country.unvisit_city(city);
        return;
    }

    for road in routes.unwrap()//order of roads doesn't matter, all need to be tried
    {
        
        roads_taken.push(road.clone());
        travel_all_roads(country.clone(), &road.get_destination_city(city), roads_taken, routes_tested, shortest_road, longest_road);
    }
    roads_taken.pop();
    country.unvisit_city(city);
}