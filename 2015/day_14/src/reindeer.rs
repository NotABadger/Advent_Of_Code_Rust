

pub struct Reindeer
{
    name: String,
    speed: u32,
    stamina_sec: u32,
    rest_sec: u32, 
    total_traveled: u32,
    state: State,
    points: u32,
}

enum State
{
    Running(u32),
    Resting(u32),
}

impl Reindeer{
    pub fn new(name: &str, speed: u32, stamina_sec: u32, rest_sec: u32) -> Self
    {
        Self{name: name.to_string(), speed, stamina_sec, rest_sec, total_traveled: 0, state: State::Running(0), points: 0}
    }

    pub fn get_distance_traveled(&self) -> u32
    {
        self.total_traveled
    }
    pub fn get_name(&self) -> String
    {
        self.name.to_string()
    }

    pub fn add_second(&mut self)
    {
        match self.state {
            State::Resting(val) => {
                if val < self.rest_sec
                {
                    self.state = State::Resting(val + 1);
                }
                else 
                {//equal or larger, switch to running
                    self.state = State::Running(1);
                    self.total_traveled += self.speed;
                }
            },
            State::Running(val) => {
                if val < self.stamina_sec
                {
                    self.total_traveled += self.speed;
                    self.state = State::Running(val+1);
                }
                else 
                {//equal or larger, switch to resting
                    self.state = State::Resting(1);
                }
            },
        }
    }

    pub fn add_point(&mut self)
    {
        self.points +=1;
    }
    pub fn get_points(&self) -> u32
    {
        self.points
    }
}