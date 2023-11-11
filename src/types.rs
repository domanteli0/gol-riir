use derive_more::IsVariant;

impl From<u64> for CellState {
    fn from(value: u64) -> Self {
        if value == 0 {
            CellState::Dead
        } else if value == 1 {
            CellState::Alive
        } else {
            // CellState::Alive
            panic!("we speak ones and zeroes only")
        }
    }
}

impl From<bool> for CellState {
   fn from(value: bool) -> Self {
        match value {
            true => CellState::Alive,
            false => CellState::Dead,
        }
   } 
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, IsVariant)]
pub enum CellState {
    Alive = 0,
    Dead = 1,
}