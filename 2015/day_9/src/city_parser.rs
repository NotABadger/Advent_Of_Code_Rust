use crate::city::City;

pub fn parse_cities(list_of_cities : &str) -> Vec<City>
{
    let mut cities: Vec<City> = Vec::new();
    for line in list_of_cities.lines()
    {
        let mut word_itt = line.split_whitespace();
        let start_city_name = word_itt.next().expect("Expecting first city name");
        let found_city_ref: &mut City;
        match cities.iter_mut().find(|listed_city| listed_city.get_name() == start_city_name)
        {
            Some(found_city) => found_city_ref = found_city,
            None =>{
                cities.push(City::default());
                found_city_ref = cities.last_mut().unwrap();
                found_city_ref.set_name(start_city_name);
            }, 
        }

        word_itt.next(); // the word "to"
        let destination_city_name = word_itt.next().expect("Expecting next city");
        word_itt.next(); // equals sign
        let distance_str = word_itt.next().expect("expecting distance");
        let distance: u32 = distance_str.parse::<u32>().unwrap();
        found_city_ref.add_city_relation(destination_city_name, distance);
    }


    cities
}