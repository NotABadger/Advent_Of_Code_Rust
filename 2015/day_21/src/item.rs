#[derive(Clone, Copy, Debug)]
pub struct Item{
    pub itemtype: ItemType,
    pub cost: u32,
    pub damage: u32,
    pub armor: u32,
}

#[derive(Clone, Copy,  Debug)]
pub enum ItemType{
    WEAPON,
    ARMOR,
    RING,
}

impl Item{
    pub fn new_weapon(cost: u32, damage: u32) ->Self{
        Self { itemtype: ItemType::WEAPON, cost, damage, armor: 0 }
    }
    pub fn new_armor(cost: u32, armor: u32) ->Self{
        Self { itemtype: ItemType::ARMOR, cost, damage: 0, armor}
    }
    pub fn new_ring(cost: u32, armor: u32, damage: u32) ->Self{
        Self { itemtype: ItemType::RING, cost, damage, armor}
    }
}