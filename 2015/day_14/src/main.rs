mod reindeer;

use crate::reindeer::Reindeer;

fn main() {
    println!("Hello, world!");

    let mut sleigh_pullers: Vec<Reindeer> = Vec::new();
    const SECONDS_RUNNING: u32 = 2503;

    //too lazy to write file parser for 8 reindeer
    sleigh_pullers.push(Reindeer::new("Rudolph", 22, 8, 165));
    sleigh_pullers.push(Reindeer::new("Cupid", 8, 17, 114));
    sleigh_pullers.push(Reindeer::new("Prancer", 18, 6, 103));
    sleigh_pullers.push(Reindeer::new("Donner", 25, 6, 145));
    sleigh_pullers.push(Reindeer::new("Dasher", 11, 12, 125));
    sleigh_pullers.push(Reindeer::new("Comet", 21, 6, 121));
    sleigh_pullers.push(Reindeer::new("Blitzen", 18, 3, 50));
    sleigh_pullers.push(Reindeer::new("Vixen", 20, 4, 75));
    sleigh_pullers.push(Reindeer::new("Dancer", 7, 20, 119));

    for _i in 0..SECONDS_RUNNING
    {
       sleigh_pullers.iter_mut().for_each(|reindeer| reindeer.add_second());
       // Part two \\
        let mut longest_distance: u32 = 0;
        sleigh_pullers.iter().for_each(|reindeer| {if reindeer.get_distance_traveled() > longest_distance { longest_distance = reindeer.get_distance_traveled()}} );
        for deer in sleigh_pullers.iter_mut()
        {
            if deer.get_distance_traveled() == longest_distance
            {
                deer.add_point();
            }
        }
        // End of part two \\
    }

    //let winner = sleigh_pullers.iter().max_by(|&r1, &r2| r1.get_distance_traveled().cmp(&r2.get_distance_traveled()));
    let winner = sleigh_pullers.iter().max_by(|&r1, &r2| r1.get_points().cmp(&r2.get_points())); //part two
    match winner
    {
        None => println!("No winner found"),
        Some(val) => println!("The winner was {:?}, and has {:?} points", val.get_name(), val.get_points()), // part two
        //Some(val) => println!("The winner was {:?}, and traveled {:?}", val.get_name(), val.get_distance_traveled()),
    }
}
