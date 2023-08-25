use std::rc::Rc;
use crate::country::Country;
use crate::route::Route;

/*  This algoritm will take all roads and sort them on lenghth 
    Then the roads will be group the cities in a province if the city hasn't been visited twice or more (incomming & outgoing road)
    Groups (provinces) will be merged when a road connects two provinces at the ends*/

pub fn resolve_traveler_problem(country: Rc<Country>) -> Option<u32>
{
    let mut routes_taken: Vec<Route> = Vec::new();
    let mut province_id_gen: u32 = 0;

    let shortest_roads = country.get_roads_sorted_shortest_distance();

    for route in &shortest_roads
    {
        let city1: String = route.connecting_cities.0.clone();
        let city2: String = route.connecting_cities.1.clone();
        if both_cities_no_provinces(&city1, &city2, country.clone(), &mut province_id_gen)
        {
            routes_taken.push(route.clone());
            continue;
        }
        if one_city_a_province(&city1, &city2, country.clone())
        {
            routes_taken.push(route.clone());
            continue;
        }
        let return_val = both_cities_a_province(&city1, &city2, country.clone());
        if return_val.is_some()
        {
            if return_val.unwrap() == 1
            {
                routes_taken.push(route.clone());
            }
            continue;
        }
    }

    if country.get_amount_visited_countries() == country.cities.len() as u32
    {
        let mut shortest_path_all_cities_val: u32 = 0;
        routes_taken.iter().for_each(|route| shortest_path_all_cities_val += route.get_distance());
        // println!("Routes taken: ");
        // println!("{:#?}", routes_taken);
        country.reset_all_travel();
        return Some(shortest_path_all_cities_val);
    }
    
    None
}


fn both_cities_no_provinces(city1: &str, city2: &str, country: Rc<Country>, province_id_gen: &mut u32) -> bool
{//Create province, add to both cities, road is now used.
    if country.get_city_province(&city1).is_none() && country.get_city_province(&city2).is_none()
    {
        //create province number, add it to cities
        //no province = no connections
        country.visit_city(city1);
        country.visit_city(city2);
        country.set_city_province(city1, *province_id_gen);
        country.set_city_province(city2, *province_id_gen);
        *province_id_gen += 1;
        return true;
    }
    return false;
}

fn one_city_a_province(city1: &str, city2: &str, country: Rc<Country>) -> bool
{//Copy province setting to other city, road now used
    let city_1_prov = country.get_city_province(city1);
    let city_2_prov = country.get_city_province(city2);
    if city_1_prov.is_some() && city_2_prov.is_none()
    {
        if country.get_citys_visit_count(city1) > 1
        {
            return false;
        }
        let province_id = city_1_prov.expect("Already checked province in if statement");
        country.set_city_province(city2, province_id);
        country.visit_city(city1);
        country.visit_city(city2);
        return true;
    }

    if city_1_prov.is_none() && city_2_prov.is_some() 
    {
        if country.get_citys_visit_count(city2) > 1
        {
            return false;
        }
        let province_id = city_2_prov.expect("Already checked province in if statement");
        country.set_city_province(city1, province_id);
        country.visit_city(city1);
        country.visit_city(city2);
        return true;
    }
    false
}

fn both_cities_a_province(city1: &str, city2: &str, country: Rc<Country>) -> Option<u8>
{
    /*When two cities have different provinces: All cities from one province are added to the other (merging two provinces), road is used 
        When two have the same province: road is not used, return true with no action taken*/ 
    let city_1_prov = country.get_city_province(city1);
    let city_2_prov = country.get_city_province(city2);
    if city_1_prov.is_some() && city_2_prov.is_some()
    {
        if country.get_citys_visit_count(city1) > 1 || country.get_citys_visit_count(city2) > 1
        {
            return None;
        }
        let city_1_prov_id = city_1_prov.unwrap();
        let city_2_prov_id = city_2_prov.unwrap();
        if city_1_prov_id == city_2_prov_id
        { //both same province
            return Some(0);
        }
        country.merge_two_provinces(city_1_prov_id, city_2_prov_id);
        country.visit_city(city1);
        country.visit_city(city2);
        return Some(1);
    }
    None
}