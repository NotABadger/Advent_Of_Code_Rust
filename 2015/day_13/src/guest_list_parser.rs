
use crate::guests::Guests;
use crate::person::Person;
use crate::relation::Relation;

pub fn parse_guest_list(list_of_relations: &str) -> Guests
{
    let mut guests: Guests = Guests::default();
    for line in list_of_relations.lines()
    {
        let mut word_itt: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let person1: &str = word_itt.next().expect("expecting first person name");
        word_itt.next();
        let gain_lose: &str = word_itt.next().expect("expecting the word gain or lose");
        let score_str: &str = word_itt.next().expect("string containing numeric happiness number");
        let mut score: i32 = score_str.parse::<i32>().expect("Should be number");
        let person2_with_dot: &str = word_itt.last().expect("Expexting second person name");
        let person2 = &person2_with_dot[0..person2_with_dot.len()-1];

        if gain_lose == "lose"
        {
            score = score * -1;
        }


        if !guests.is_guest_known(person1)
        {
            guests.people.insert(person1.to_string(), Person::new(person1));
        }
        if !guests.is_guest_known(person2)
        {
            guests.people.insert(person2.to_string(), Person::new(person2));
        }

        if guests.is_relation_known(person1, person2)
        {
            let existing_relation: &Relation;
            existing_relation = guests.get_relation(person1, person2);
            existing_relation.add_happiness(score);
        }
        else 
        {
            let new_relation: Relation = Relation::new(person1, person2, score);
            guests.relations.push(new_relation);
        }
    }

    //part two, add myself in solution
    {
        let myself = "Mathieu";
        for guest in &guests.people
        {
            guests.relations.push(Relation::new(myself, &guest.0, 0));
        }
        guests.people.insert(myself.to_string(), Person::new(myself));

    }

    //println!("guests: {:#?}", guests.people);
    //println!("relations: {:#?}", guests.relations);
    guests
}