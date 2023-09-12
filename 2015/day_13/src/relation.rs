use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Relation
{
    people_in_relation:(String,String),
    happiness_score: Cell<i32>,
}

impl Relation
{
    pub fn new(person_1: &str, person_2: &str, happiness: i32) -> Self
    {
        Self{people_in_relation: (person_1.to_string(), person_2.to_string()), happiness_score: Cell::new(happiness)}
    }

    pub fn get_happiness_score(&self) -> i32
    {
        self.happiness_score.get()
    }

    pub fn person_in_relation(&self, person: &str) -> bool
    {
        if self.people_in_relation.0 == person || self.people_in_relation.1 == person
        {
            return true;
        }
        false
    }

    pub fn add_happiness(&self, score_to_add: i32)
    {
        let mut score: i32 = self.happiness_score.get();
        score += score_to_add;
        self.happiness_score.set(score);
    }

    pub fn get_other_person_in_relation(&self, person: &str) -> String
    {
        if self.people_in_relation.0 == person
        {
            return self.people_in_relation.1.to_string();
        }
        return self.people_in_relation.0.to_string();
    }
}
