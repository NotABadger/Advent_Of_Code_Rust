#[derive(Copy, Clone)]
pub struct Light
{
    emitting_light: u32,
}

impl Light {
    pub fn new() -> Self
    {
        return Self{emitting_light: 0};
    }
    pub fn turn_on(&mut self)
    {
        self.emitting_light +=1;
    }

    pub fn turn_off(&mut self)
    {
        if self.emitting_light > 0
        {
            self.emitting_light -= 1;
        }
    }

    pub fn toggle(&mut self)
    {
        self.emitting_light += 2;
    }

    pub fn current_state(&self) -> u32
    {
        return self.emitting_light;
    }
}
