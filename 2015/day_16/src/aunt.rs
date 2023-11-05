#[derive(Debug, Clone)]
pub struct Aunt{
    pub aunt_nr: Option<u32>,
    pub children: Option<u32>,
    pub cats: Option<u32>,
    pub samoyeds: Option<u32>,
    pub pomeranians: Option<u32>,
    pub akitas: Option<u32>,
    pub vizslas: Option<u32>,
    pub goldfish: Option<u32>,
    pub trees: Option<u32>,
    pub cars: Option<u32>,
    pub perfumes: Option<u32>,
}

impl Aunt{
    pub fn set_parameter(&mut self, prop: &str, value: u32)
    {
        match prop {
            "Sue" =>  self.aunt_nr = Some(value),
            "children" => self.children = Some(value),
            "cats" => self.cats = Some(value),
            "samoyeds" => self.samoyeds = Some(value),
            "pomeranians" => self.pomeranians = Some(value),
            "akitas" => self.akitas = Some(value),
            "vizslas" => self.vizslas = Some(value),
            "goldfish" => self.goldfish = Some(value),
            "trees" => self.trees = Some(value),
            "cars" => self.cars = Some(value),
            "perfumes" => self.perfumes = Some(value),
            _ => panic!("tried to parse unknown property! prop string: {}", prop),
        }
    }

    pub fn is_possible_same(&self, guess :&Aunt) -> bool
    {
        if guess.children.is_some()
        {
            if self.children.expect("don't know sender") != guess.children.expect("hi")
            {
                return false;
            }
        }
        if guess.cats.is_some()
        {
            if self.cats.expect("don't know sender") != guess.cats.expect("hi")
            {
                return false;
            }
        }
        if guess.samoyeds.is_some()
        {
            if self.samoyeds.expect("don't know sender") != guess.samoyeds.expect("hi")
            {
                return false;
            }
        }
        if guess.pomeranians.is_some()
        {
            if self.pomeranians.expect("don't know sender") != guess.pomeranians.expect("hi")
            {
                return false;
            }
        }
        if guess.akitas.is_some()
        {
            if self.akitas.expect("don't know sender") != guess.akitas.expect("hi")
            {
                return false;
            }
        }
        if guess.vizslas.is_some()
        {
            if self.vizslas.expect("don't know sender") != guess.vizslas.expect("hi")
            {
                return false;
            }
        }
        if guess.goldfish.is_some()
        {
            if self.goldfish.expect("don't know sender") != guess.goldfish.expect("hi")
            {
                return false;
            }
        }
        if guess.trees.is_some()
        {
            if self.trees.expect("don't know sender") != guess.trees.expect("hi")
            {
                return false;
            }
        }
        if guess.cars.is_some()
        {
            if self.cars.expect("don't know sender") != guess.cars.expect("hi")
            {
                return false;
            }
        }
        if guess.perfumes.is_some()
        {
            if self.perfumes.expect("don't know sender") != guess.perfumes.expect("hi")
            {
                return false;
            }
        }

        
        return true;
    }
}

impl Default for Aunt{
    fn default() -> Self {
        Self { aunt_nr: None, children: None, cats: None, samoyeds: None, pomeranians: None, akitas: None, vizslas: None, goldfish: None, trees: None, cars: None, perfumes: None }
    }
}