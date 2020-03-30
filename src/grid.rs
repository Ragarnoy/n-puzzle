use std::{
    fmt,
    hash::{Hash, Hasher}
};
use utils::{snail_sort, coord::Coord};
use rand::{self, Rng};
use crate::puzzle_gen::create_snail_goal;

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
    pub fn from(from: u8) -> Self
    {
        match from
        {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Right,
            3 => Self::Left,
            _ => Self::Up
        }
    }

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

impl fmt::Display for HType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            Self::Hamming => write!(f, "Hamming"),
            Self::Manhattan => write!(f, "Manhattan"),
            Self::LinearManhattan => write!(f, "Linear-Manhattan")
        }
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

#[derive(Clone, Debug)]
pub struct Grid
{
    map: Vec<u16>,
    z_pos: u16,
    lines: u8,
}

impl Hash for Grid
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.map.hash(state);
    }
}

impl PartialEq for Grid
{
    fn eq(&self, other: &Self) -> bool
    {
        self.map == other.map
    }
}

impl Eq for Grid {}

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

    pub fn new_random(lines: u8) -> Self
    {
        let mut rng = rand::thread_rng();
        let mut puzzle = Self::new(create_snail_goal(lines), lines);
        for i in 0..(512 * lines as u128)
        {
            puzzle = puzzle.move_zero(Move::from(rng.gen_range(0, 4))).unwrap_or(puzzle);
        }
        puzzle
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

    pub fn hamming(&self, goal: &Grid) -> u32
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

    pub fn solvable(&self) -> bool
    {
        let mut solve = self.map.clone();
        let mut inv_cout: u16 = 0;
        let ret: bool;
        let blank_bot = self.lines as i16 - Coord::from_abs(solve.iter().position(|&x| x == 0).unwrap_or(0) as u32, self.lines).y;

        while !solve.is_empty()
        {
            let i = solve.remove(0);
            if i == 0
            {
                continue;
            }
            for j in solve.iter().filter(|j| **j < i)
            {
                if *j != 0
                {
                    inv_cout += 1;
                }
            }
        }
        
        if self.lines % 2 == 0
        {
            if blank_bot % 2 == 0
            {
                ret = inv_cout % 2 != 0;
            }
            else
            {
                ret = !(inv_cout % 2 != 0);
            }
        }
        else
        {
            ret = inv_cout % 2 != 0;
        }
    if self.lines < 6
    {
        ret
    }
    else
    {
        !ret
    }
    }

    pub fn manhattan(&self, goal: &Grid) -> u32
    {
        self.map.iter().zip(goal.map.iter()).filter(|(i, _)| **i != 0).fold(0, |acc, (i, g)| 
        {
            let goal_cord = Coord::from_abs(goal.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, self.lines);
            let self_cord = Coord::from_abs(self.map.iter().enumerate().find(|(_, y)| **y == *i).unwrap().0 as u32, self.lines);
            if i != g
            {
                acc + ((goal_cord.x - self_cord.x).abs() as u32 + (goal_cord.y - self_cord.y).abs() as u32)
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

    pub fn linear_conflict(&self, goal: &Grid) -> u32
    {
        let mut ret: u32 = 0;
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

    pub fn linear_manhattan(&self, goal: &Grid) -> u32
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
            result = match x
            {
                0 => write!(f, "\t{}", "_")?,
                n => write!(f, "\t{}", n)?
            };
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

    #[test]
    fn solvable()
    {
        let test = Grid::new(vec!(
                                0, 7, 6, 
                                8, 2, 4, 
                                1, 5, 3), 3);
        println!("{}", test.solvable());
        let test = Grid::new(vec!(
                                0, 1, 3, 
                                4, 8, 6, 
                                2, 5, 7), 3);
        println!("{}", test.solvable());
        let test = Grid::new(vec!(
                                2, 1, 0, 
                                4, 6, 5, 
                                8, 7, 3), 3);
        println!("{}", test.solvable());
        let test = Grid::new(vec!(
                                2, 5, 0, 
                                8, 7, 6, 
                                3, 4, 1), 3);
        println!("{}", test.solvable());
        // Below is solvable
        let test = Grid::new(vec!(
                                1, 0, 3, 
                                8, 2, 4, 
                                7, 6, 5), 3);
        println!("{}", test.solvable());
        let test = Grid::new(vec!(
                                1, 2, 3, 
                                8, 4, 0, 
                                7, 6, 5), 3);
        println!("{}", test.solvable());
    }

    #[test]
    fn new_random()
    {
        let puzzle = Grid::new_random(4);
        println!("{}", puzzle);
    }
}
