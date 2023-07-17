use crate::House;

pub struct DeliveryBoy{
    cur_x_cor: i32,
    cur_y_cor: i32,
}

pub fn create_delivery_boy() -> DeliveryBoy
{
    return DeliveryBoy {cur_x_cor: 0, cur_y_cor: 0,};
}

impl DeliveryBoy{
    
    pub fn do_instruction(&mut self, instruction: &char, map: &mut Vec<House>)
    {
        match instruction{
            '^' => self.cur_y_cor += 1,
            'v' => self.cur_y_cor -= 1,
            '>' => self.cur_x_cor += 1,
            '<' => self.cur_x_cor -= 1,
            _ => (),
        }
        let mut house_exists: bool = false;
        for house in &mut *map
        {
            if house.coordinates == (self.cur_x_cor, self.cur_y_cor)
            {
                house.amount_pres += 1;
                house_exists = true;
                break;
            }
        }
        if !house_exists
        {
            map.push(House{coordinates: (self.cur_x_cor.clone(), self.cur_y_cor.clone()),amount_pres: 1});
        }

        
    }
}