mod race;

use race::Race;

fn main() {
    println!("Racing!");
    let races: Vec<Race> = vec![Race::new(62, 553),
                                    Race::new(64, 1010),
                                    Race::new(91, 1473),
                                    Race::new(90, 1074),];
    let mut margin_error: u64 = 1;
    for race in &races {
        margin_error = margin_error * race.different_ways_of_winning();
    }

    println!("Margin of error in small races: {}",margin_error);

    let long_race: Race = Race::new(62649190, 553101014731074);
    println!("Margin of error in long race: {}",long_race.different_ways_of_winning_long_race());
}
 /* 
    Time:        62     64     91     90
    Distance:   553   1010   1473   1074
 */