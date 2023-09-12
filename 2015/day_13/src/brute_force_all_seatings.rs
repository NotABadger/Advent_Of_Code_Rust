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

    for itt in guest_list.people.iter()
    {//take every person/guest as starting point
        let mut seating_order: Vec<Relation> = Vec::new();       
        try_all_seating_combos(guest_list.clone(), &itt.0, &mut seating_order,&mut seatings_tested, &mut best_seating, &mut worst_seating, itt.0);
        itt.1.reset_seating();
    }

    println!("Amount of seating combinations tested: {:?}", seatings_tested);
    println!("Brute force found best seating: {:?}", best_seating);
    println!("Brute force found worst seating: {:?}", worst_seating);
}

pub fn try_all_seating_combos(guest_list: Rc<Guests>, guest: &str, seating_order: &mut Vec<Relation>, seatings_tested: &mut u32, best_seating: &mut i32, worst_seating: &mut i32, first_seated: &str)
{
    /*  Seat the given guest, and retrieve remaining possible relations that with guests that are not yet seated
        when no more guests/relations are available, print result of seating arrangement
        When more guests are available, try all combinations */
    guest_list.seat_person(guest);
    
    let possible_relations = guest_list.get_remaining_guest_relations(guest);
    if possible_relations.is_none()
    {
        let mut total_happiness : i32 = 0 ;

        if guest_list.is_relation_known(first_seated, guest)
        {
            total_happiness += guest_list.get_relation(first_seated, guest).get_happiness_score();
            seating_order.iter().for_each(|relation| total_happiness += relation.get_happiness_score());
            if total_happiness > *best_seating
            {
                *best_seating = total_happiness;
            }
            if total_happiness < *worst_seating
            {
                *worst_seating = total_happiness;
            }
            *seatings_tested += 1; 
        }
        else 
        {
            println!("last relation could not be found. {:0}, {:1}", first_seated, guest);
        }

        guest_list.unseat_guest(guest);
        return;
    }

    for relation in possible_relations.unwrap()//order of relations doesn't matter, all need to be tried
    {
        seating_order.push(relation.clone());
        try_all_seating_combos(guest_list.clone(), &relation.get_other_person_in_relation(guest), seating_order, seatings_tested, best_seating, worst_seating, first_seated);
        seating_order.pop();
    }
    guest_list.unseat_guest(guest);
}