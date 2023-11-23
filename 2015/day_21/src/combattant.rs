use crate::item::Item;

#[derive(Debug, Clone)]
pub struct Combattant{
    pub name: String,
    pub max_hp: i32,
    pub current_hp: i32,
    pub damage: u32,
    pub armor: u32,
    pub armor_item: Item,
    pub weapon_item: Item,
    pub ring_items: [Item; 2],
}

impl Combattant{
    pub fn reset_hp(&mut self){
        self.current_hp = self.max_hp;
    }

    pub fn update_stats(&mut self) {
        self.damage = 0; 
        self.armor = 0;
        
        self.damage += self.weapon_item.damage;
        self.ring_items.iter().for_each(|ring| self.damage += ring.damage );

        self.armor += self.armor_item.armor;
        self.ring_items.iter().for_each(|ring| self.armor += ring.armor );
    }

    pub fn get_total_value(&self) -> u32
    {
        self.armor_item.cost + self.weapon_item.cost + self.ring_items[0].cost + self.ring_items[1].cost
    }
}

impl Default for Combattant{
    fn default() -> Self {
        Self { name: "Bob".to_string(), 
                max_hp: 100, 
                current_hp: 100,
                damage: 0, 
                armor: 0, 
                armor_item: Item::new_armor(0, 0), 
                weapon_item: Item::new_weapon(0, 0), 
                ring_items: [Item::new_ring(0, 0, 0), Item::new_ring(0, 0, 0)] }
    }
}