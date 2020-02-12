use std::cmp::{Ordering, Ord};
use crate::grid::Grid;
#[derive(Eq, PartialEq, PartialOrd, Clone)]
pub struct State
{
    h:u16,
    pub g:u32,
    f:u32,
}

// TODO Generate h from grid method that executes heuristic funtion

impl Ord for State
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if self.f > other.f
        {
            Ordering::Greater
        }
        else if self.f < other.f
        {
            Ordering::Less
        }
        else
        {
            if (self.h > other.h) || (self.h == other.h && self.g > other.g)
            {
                Ordering::Greater
            }
            else if (self.h < other.h) || (self.h == other.h && self.g < other.g)
            {
                Ordering::Less
            }
            else
            {
                Ordering::Equal
            }
        }
    }
}

impl State
{
    pub fn new(h:u16, g:u32, f:u32) -> Self
    {
        State
        {
            h,
            g,
            f
        }
    }
    
    pub fn update(&self, grid: Vec<u16>, goal: Vec<u16>) -> State
    {
        let (h, g) = (manning(grid, goal), self.g + 1);
        State
        {
            h,
            g,
            f: g + h as u32,
        }
    }
}



fn manning(input: Vec<u16>, goal: Vec<u16>) -> u16
{
    let mut ret:u16 = 0;

    ret = input.iter().zip(goal.iter()).filter(|(i, _)| **i != 0).fold(0, |acc, (i, g)| 
    {
        if i != g 
        {
            acc + 1
        }
        else
        {
            acc
        }
    });
    ret
}

// fn manhattan(input: Vec<Vec<u16>>, goal: Vec<Vec<u16>>)
// {

// }

// fn linear_manhattan(input: Vec<Vec<u16>>, goal: Vec<Vec<u16>>)
// {

// }


#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_cmp_greater_f()
    {
        let small = State{h: 1, g: 1, f: 1};
        let big = State{h: 3, g: 1, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Greater);
    }
    #[test]
    fn test_cmp_lesser_f()
    {
        let small = State{h: 1, g: 1, f: 1};
        let big = State{h: 3, g: 1, f: 5};
        assert_eq!(small.cmp(&big), Ordering::Less);
    }
    #[test]
    fn test_cmp_greater_h()
    {
        let small = State{h: 1, g: 2, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Greater);
    }
    #[test]
    fn test_cmp_lesser_h()
    {
        let small = State{h: 1, g: 2, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert_eq!(small.cmp(&big), Ordering::Less);
    }
    #[test]
    fn test_cmp_greater_g()
    {
        let small = State{h: 3, g: 1, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Greater);
    }
    #[test]
    fn test_cmp_equal()
    {
        let small = State{h: 1, g: 1, f: 5};
        let big = State{h: 1, g: 1, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Equal);
    }
}
