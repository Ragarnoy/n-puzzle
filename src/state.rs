use std::cmp::{Ordering, Ord};
use crate::grid::Grid;
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct State
{
    pub h: u16,
    pub g: u32,
    pub f: u32,
}

impl PartialOrd for State
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(if other.f > self.f
        {
            Ordering::Greater
        }
        else if other.f < self.f
        {
            Ordering::Less
        }
        else
        {
            if (other.h > self.h) || (other.h == self.h && other.g > self.g)
            {
                Ordering::Greater
            }
            else if (other.h < self.h) || (other.h == self.h && other.g < self.g)
            {
                Ordering::Less
            }
            else
            {
                Ordering::Equal
            }
        })
    }
}

impl Ord for State
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        if other.f > self.f
        {
            Ordering::Greater
        }
        else if other.f < self.f
        {
            Ordering::Less
        }
        else
        {
            if (other.h > self.h) || (other.h == self.h && other.g > self.g)
            {
                Ordering::Greater
            }
            else if (other.h < self.h) || (other.h == self.h && other.g < self.g)
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
    
    pub fn update_manning(&mut self, grid: &Grid, goal: &Grid)
    {
        self.h = grid.manning(goal);
        self.f = self.g + self.h as u32;
    }
    
    pub fn update_manhattan(&mut self, grid: &Grid, goal: &Grid, col: u8)
    {
        self.h = grid.manhattan(goal, col);
        self.f = self.g + self.h as u32;
    }
    
    pub fn update_linear_manhattan(&mut self, grid: &Grid, goal: &Grid, col: u8)
    {
        self.h = grid.linear_manhattan(goal, col);
        self.f = self.g + self.h as u32;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_cmp_greater_f()
    {
        let small = State{h: 1, g: 1, f: 1};
        let big = State{h: 3, g: 1, f: 5};
        assert!(small > big);
        assert_eq!(big.cmp(&small), Ordering::Less);
    }
    #[test]
    fn test_cmp_lesser_f()
    {
        let small = State{h: 1, g: 1, f: 1};
        let big = State{h: 3, g: 1, f: 5};
        assert!(small > big);
        assert_eq!(small.cmp(&big), Ordering::Greater);
    }
    #[test]
    fn test_cmp_greater_h()
    {
        let small = State{h: 1, g: 2, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert!(small > big);
        assert_eq!(big.cmp(&small), Ordering::Less);
    }
    #[test]
    fn test_cmp_lesser_h()
    {
        let small = State{h: 1, g: 2, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert_eq!(small.cmp(&big), Ordering::Greater);
    }
    #[test]
    fn test_cmp_greater_g()
    {
        let small = State{h: 3, g: 1, f: 5};
        let big = State{h: 3, g: 2, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Less);
    }
    #[test]
    fn test_cmp_equal()
    {
        let small = State{h: 1, g: 1, f: 5};
        let big = State{h: 1, g: 1, f: 5};
        assert_eq!(big.cmp(&small), Ordering::Equal);
    }
}
