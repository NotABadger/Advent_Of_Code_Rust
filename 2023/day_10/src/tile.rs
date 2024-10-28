use crate::directions::Directions;
use crate::error::MyFuckingError;

#[derive(Debug, PartialEq, Eq)]
pub enum TileType {
    Ground,
    Pipe(PipeType),
    StartPos,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PipeType {
    Vertical,
    Horizontal,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
}

impl PipeType {
    pub fn determine_outgoing_direction(&self, incomming_dir: Directions) -> Result<Directions, MyFuckingError> {
        match self {
            Self::Vertical => {if incomming_dir != Directions::North && incomming_dir != Directions::South {
                return Err(MyFuckingError::new("Vertical pipes can only be traveled N <-> S"));
                }
                Ok(incomming_dir)
            }
            Self::Horizontal => {if incomming_dir != Directions::West && incomming_dir != Directions::East {
                return Err(MyFuckingError::new("Horizontal pipes can only be traveled W <-> E"));
                }
                Ok(incomming_dir)
            }
            Self::CornerNE => {if incomming_dir != Directions::South && incomming_dir != Directions::West {
                return Err(MyFuckingError::new("Corner North-East pipes can only be traveled N <-> E"));
                }
                if incomming_dir == Directions::South {return Ok(Directions::East);}
                Ok(Directions::North)
            }
            Self::CornerSE => {if incomming_dir != Directions::North && incomming_dir != Directions::West {
                return Err(MyFuckingError::new("Corner South-East pipes can only be traveled S <-> E"));
                }
                if incomming_dir == Directions::North {return Ok(Directions::East);}
                Ok(Directions::South)
            }
            Self::CornerNW => {if incomming_dir != Directions::South && incomming_dir != Directions::East {
                return Err(MyFuckingError::new("Corner North-West pipes can only be traveled N <-> W"));
                }
                if incomming_dir == Directions::South {return Ok(Directions::West);}
                Ok(Directions::North)
            }
            Self::CornerSW => {if incomming_dir != Directions::North && incomming_dir != Directions::East {
                return Err(MyFuckingError::new("Corner South-West pipes can only be traveled S <-> W"));
                }
                if incomming_dir == Directions::North {return Ok(Directions::West);}
                Ok(Directions::South)
            }
        }
    }
}

impl From<char> for TileType {
   fn from(value: char) -> Self {
    match value {
        '.' => TileType::Ground,
        'S' => TileType::StartPos,
        _ => {
            if let Ok(pipe) = PipeType::try_from(value) {
                TileType::Pipe(pipe)
            }
            else {
                TileType::Ground
            }
        },
    }
   }
}

impl TryFrom<char> for PipeType {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(PipeType::Vertical),
            '-' => Ok(PipeType::Horizontal),
            'L' => Ok(PipeType::CornerNE),
            'J' => Ok(PipeType::CornerNW),
            '7' => Ok(PipeType::CornerSW),
            'F' => Ok(PipeType::CornerSE),
            _ => Err("Not a Pipe"),
        }
    }
}