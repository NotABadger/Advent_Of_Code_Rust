use crate::country::Country;
use crate::city::City;
use crate::route::Route;

pub fn parse_country(list_of_cities : &str) -> Country
{
    let mut country: Country = Country::default();
    for line in list_of_cities.lines()
    {
        let mut word_itt: std::str::SplitWhitespace<'_> = line.split_whitespace();

        let start_city_name: &str = word_itt.next().expect("Expecting first city name");
        word_itt.next(); // the word "to"
        let destination_city_name = word_itt.next().expect("Expecting next city");
        word_itt.next(); // equals sign
        let distance_str: &str = word_itt.next().expect("expecting distance");
        let distance: u32 = distance_str.parse::<u32>().unwrap();

        if !country.is_city_known(start_city_name)
        {
            country.cities.insert(start_city_name.to_string(), City::new());
        }
        if !country.is_city_known(destination_city_name)
        {
            country.cities.insert(destination_city_name.to_string(), City::new());
        }
        if !country.is_route_known(start_city_name, destination_city_name)
        {
            country.routes.push(Route::new(start_city_name, destination_city_name, distance));
        }
    }
    country
}