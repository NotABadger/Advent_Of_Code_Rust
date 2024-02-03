use crate::effect_trait::Effect;

#[derive(Debug)]
pub struct MagicMissile {
    name: String,
}

impl Effect for MagicMissile {
     //get name of spell
     fn get_name(&self) -> String {
        self.name.clone()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        53
    }
    //get damage an effect does
    fn get_dmg(&self) -> i32 {
        4
    }    
}

impl MagicMissile {
    const NAME: &str = "Magic Missile";

    pub fn new() -> Self {
        Self{name: Self::NAME.to_string()}
    }
}