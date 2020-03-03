use std::fmt;
use utils::coord::Coord;

#[derive(Copy, Clone, Debug)]
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
            Move::Up => coord.dec_y(),
            Move::Down => coord.inc_y(),
            Move::Right => coord.inc_x(),
            Move::Left => coord.dec_x()
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HType
{
    Hamming,
    Manhattan,
    LinearManhattan
}

impl Default for HType
{
    fn default() -> Self
    {
        Self::Hamming
    }
}

impl HType
{
    pub fn from_str_or_default(input: Option<&str>) -> Result<Self, String>
    {
        match input
        {
            None => Ok(Self::default()),
            Some("hamming") => Ok(Self::Hamming),
            Some("manhattan") => Ok(Self::Manhattan),
            Some("linear_manhattan") => Ok(Self::LinearManhattan),
            Some(h) => Err(format!("This heuristic function does not exist: {}", h))
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Grid
{
    map: Vec<u16>,
    z_pos: u16,
    lines: u8,
}

impl Grid
{
    pub fn new(map: Vec<u16>, lines: u8) -> Self
    {
        Self
        {
            z_pos: map.iter().position(|&x| x == 0).unwrap_or(0) as u16,
            map,
            lines
        }
    }

    pub fn get_lines(&self) -> u8
    {
        self.lines
    }

    pub fn get_map(&self) -> Vec<u16>
    {
        self.map.clone()
    }

    pub fn move_zero(&self, mov: Move) -> Option<Self>
    {
        let mut z_pos = Coord::from_abs(self.z_pos as u32, self.lines);
        let mut map = self.map.clone();

        mov.apply(&mut z_pos);
        if z_pos.is_out_of_table(self.lines)
        {
            return None;
        }

        let z_pos = z_pos.to_abs(self.lines);
        map[self.z_pos as usize] = map[z_pos as usize];
        map[z_pos as usize] = 0;
        Some(Self
        {
            map,
            lines: self.lines,
            z_pos: z_pos as u16
        })
    }

    pub fn move_all_possible(&self) -> Vec<Self>
    {
        [Move::Up, Move::Down, Move::Right, Move::Left].iter().filter_map(|&m| self.move_zero(m)).collect()
    }

    pub fn hamming(&self, goal: &Grid) -> u16
    {
        self.map.iter().zip(goal.map.iter()).filter(|(i, _)| **i != 0).fold(0, |acc, (i, g)| 
        {
            if i != g 
            {
                acc + 1
            }
            else
            {
                acc
            }
        })
    }

    pub fn manhattan(&self, goal: &Grid) -> u16
    {
        self.map.iter().zip(goal.map.iter()).filter(|(i, _)| **i != 0).fold(0, |acc, (i, g)| 
        {
            let goal_cord = Coord::from_abs(goal.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, self.lines);
            let self_cord = Coord::from_abs(self.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, self.lines);
            if i != g
            {
                acc + ((goal_cord.x - self_cord.x).abs() + (goal_cord.y - self_cord.y).abs()) as u16
            }
            else
            {
                acc
            }
        })
    }

    fn check_misplaced(from: &Grid, goal: &Grid) -> Vec<(Coord, Coord)>
    {
        from.map.iter().zip(goal.map.iter()).filter(|(i, _)| **i != 0).filter_map(|(i, g)| 
        {
            let goal_cord = Coord::from_abs(goal.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, from.lines);
            let self_cord = Coord::from_abs(from.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, from.lines);
            if i != g 
            {
                Some((self_cord, goal_cord))
            }
            else
            {
                None
            }
        }).collect()
    }

    pub fn linear_conflict(&self, goal: &Grid) -> u16
    {
        let mut ret: u16 = 0;
        let mut conflict: Vec<(Coord, Coord)> = Grid::check_misplaced(self, goal).into_iter().filter(|(from, goal)| from.x == goal.x || from.y == goal.y).rev().collect();

        while let Some((f, g)) = conflict.pop()
        {
            for (o, p) in conflict.iter().filter(|(o, p)| (f.x == g.x && f.x == o.x && g.x == p.x) || (f.y == g.y && f.y == o.y && g.y == p.y))
            {
                if (g.x - p.x) > 0 || (g.y - p.y) > 0
                {
                    ret += 1;
                }
            }
        }
        ret
    }

    pub fn linear_manhattan(&self, goal: &Grid) -> u16
    {
        self.manhattan(goal) + self.linear_conflict(goal) * 2
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
    fn display()
    {
        let test = Grid::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 0), 3);
        println!("{}", test);
        let test = Grid::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0), 4);
        println!("{}", test);
    }

    #[test]
    fn hamming()
    {
        let goal = Grid::new(vec!(1, 2, 3, 8, 0, 4, 7, 6, 5), 3);
        let test = Grid::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 0), 3);
        let expected = 4;

        assert_eq!(test.hamming(&goal), expected);
    }

    #[test]
    fn manhattan()
    {
        let goal = Grid::new(vec!(1, 2, 3, 8, 0, 4, 7, 6, 5), 3);
        let test = Grid::new(vec!(1, 2, 5, 3, 0, 6, 7, 4, 8), 3);

        let expected = 12;
        assert_eq!(test.manhattan(&goal), expected);

        let expected = 2;
        let test = Grid::new(vec!(1, 2, 3, 8, 0, 4, 7, 5, 6), 3);
        assert_eq!(test.manhattan(&goal), expected);

        let expected = 1;
        let test = Grid::new(vec!(1, 0, 3, 8, 2, 4, 7, 6, 5), 3);
        assert_eq!(test.manhattan(&goal), expected);

        let expected = 4;
        let test = Grid::new(vec!(1, 2, 3, 8, 0, 4, 5, 7, 6), 3);
        assert_eq!(test.manhattan(&goal), expected);
    }

    #[test]
    fn misplaced()
    {
        let goal = Grid::new(vec!(1, 2, 3, 8, 0, 4, 7, 6, 5), 3);
        let test = Grid::new(vec!(3, 1, 2, 8, 0, 4, 7, 6, 5), 3);

        assert_eq!(Grid::check_misplaced(&test, &goal), vec![ (Coord::new(0, 0), Coord::new(2, 0) ), 
                                                        (Coord::new(1, 0), Coord::new(0, 0) ), 
                                                        (Coord::new(2, 0), Coord::new(1, 0) ) ]);
    }

    #[test]
    fn linear_conflict()
    {
        let goal = Grid::new(vec!(
                                1, 2, 3,
                                8, 0, 4, 
                                7, 6, 5), 3);
        let test = Grid::new(vec!(
                                3, 1, 2, 
                                8, 0, 4, 
                                7, 6, 5), 3);
        assert_eq!(test.linear_conflict(&goal), 2);
        let test = Grid::new(vec!(
                                8, 2, 3, 
                                7, 0, 4, 
                                1, 6, 5), 3);
        assert_eq!(test.linear_conflict(&goal), 2);
        let test = Grid::new(vec!(
                                8, 2, 4, 
                                7, 0, 5, 
                                1, 6, 3), 3);
        assert_eq!(test.linear_conflict(&goal), 4);
        let test = Grid::new(vec!(
                                8, 6, 4, 
                                7, 0, 5, 
                                1, 2, 3), 3);
        assert_eq!(test.linear_conflict(&goal), 5);
    }
}
