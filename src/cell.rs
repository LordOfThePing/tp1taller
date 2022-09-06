
#[derive(Debug, PartialEq)]
pub enum Cell {
    Bomb,
    Empty,
    Number(u8),
}


impl Cell {
    pub fn obtain_ascii(&self) -> u8 {
        match &self {
            Cell::Bomb => 42,
            Cell::Empty => 46,
            Cell::Number(num) => {
                u8::try_from(*num).expect("Error: not all integers can be shown with 'try_from()'") + 48
            }
        }
    }

    pub fn increase(&self) -> Cell{
        match &self {
            Cell::Bomb => Cell::Bomb,
            Cell::Empty => Cell::Number(1),
            Cell::Number(num) => Cell::Number(num+1),
        }
    }

}
