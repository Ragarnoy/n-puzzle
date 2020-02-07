use std::fmt;
use utils::coord::Coord;

pub enum Move
{
    Up,
    Down,
    Right,
    Left
}

impl Move
{
    pub fn apply(&self, coord: &mut Coord)
    {
        match self
        {
            Self::Up => coord.dec_y(),
            Self::Down => coord.inc_y(),
            Self::Right => coord.inc_x(),
            Self::Left => coord.dec_x()
        };
    }
}

#[derive(Eq, PartialEq)]
pub struct Grid
{
    map: Vec<u16>,
    z_pos: u16,
}

impl Grid
{
    pub fn new(map: Vec<u16>) -> Self
    {
        Self
        {
            z_pos: map.iter().position(|&x| x == 0).unwrap_or(0) as u16,
            map
        }
    }

    pub fn move_zero(&self, mov: Move, col: u8) -> Option<Self>
    {
        let mut z_pos = Coord::from_abs(self.z_pos as u32, col);
        let mut map = self.map.clone();

        mov.apply(&mut z_pos);
        if z_pos.is_out_of_table(col)
        {
            return None;
        }

        let z_pos = z_pos.to_abs(col);
        map[self.z_pos as usize] = map[z_pos as usize];
        map[z_pos as usize] = 0;
        Some(Self
        {
            map,
            z_pos: z_pos as u16
        })
    }
}

impl fmt::Display for Grid
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let len = self.map.len();
        let col = (len as f32).sqrt() as u32;
        let mut result = ();
        for (i, x) in self.map.iter().enumerate()
        {
            result = write!(f, "\t{}", x)?;
            if (i + 1) % col as usize == 0 && i != 0
            {
                result = write!(f, "{}", "\n\n")?;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn diplay()
    {
        let test = Grid::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 0));
        println!("{}", test);
        let test = Grid::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0));
        println!("{}", test);
    }
}