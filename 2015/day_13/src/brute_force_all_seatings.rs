use crate::guests::Guests;
use crate::person::Person;
use crate::relation::Relation;
use std::rc::Rc;


/* This algoritm is almost an exact copy of day 9.
    Since this challange is the same, except that you have to combine 2 happiness scores into one route/relationship.
    Next to that the relation between the first and last should be added*/

pub fn resolve_seating_problem(guest_list: Rc<Guests>)
{
    let mut seatings_tested: u32 = 0;
    let mut best_seating: i32 = i32::MIN;
    let mut worst_seating: i32 = i32::MAX;


    for itt in guest_list.cities.iter()
    {//take every city as starting point
        let mut seating_order: Vec<Relation> = Vec::new();       
        try_all_seating_combos(country.clone(), &itt.0, &mut seating_order,&mut routes_tested, &mut shortest_route, &mut longest_route);
        country.reset_all_travel();
    }

    println!("Amount of routes tested: {:?}", seatings_tested);
    println!("Brute force found best seating: {:?}", best_seating);
    println!("Brute force found worst seating: {:?}", worst_seating);
}

pub fn try_all_seating_combos(country: Rc<Guests>, city: &str, roads_taken: &mut Vec<Relation>, routes_tested: &mut u32, shortest_road: &mut i32, longest_road: &mut i32)
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