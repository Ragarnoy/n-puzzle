use std::cmp::{Ordering, Ord};
use crate::grid::Grid;
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct State
{
    pub h: u32,
    pub g: u32,
    pub f: u64,
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
        else if (other.h > self.h) || (other.h == self.h && other.g > self.g)
        {
            Ordering::Greater
        }
        else if (other.h < self.h) || (other.h == self.h && other.g < self.g)
        {
            Ordering::Less
        }
        else {
            Ordering::Equal
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
        else if (other.h > self.h) || (other.h == self.h && other.g > self.g)
        {
            Ordering::Greater
        }
        else if (other.h < self.h) || (other.h == self.h && other.g < self.g)
        {
            Ordering::Less
        }
        else {
            Ordering::Equal
        }
    }
}

impl State
{
    pub fn new(h:u32, g:u32, f:u64) -> Self
    {
        State
        {
            h,
            g,
            f
        }
    }

    fn compute_f(&mut self, greedy: bool)
    {
        self.f = self.h as u64;
        if !greedy
        {
            self.f += self.g as u64;
        }
    }
    
    pub fn update_hamming(&mut self, grid: &Grid, goal: &Grid, weight: u32, greedy: bool)
    {
        self.h = grid.hamming(goal) * weight;
        self.compute_f(greedy);
    }
    
    pub fn update_manhattan(&mut self, grid: &Grid, goal: &Grid, weight: u32, greedy: bool)
    {
        self.h = grid.manhattan(goal) * weight;
        self.compute_f(greedy);
    }
    
    pub fn update_linear_manhattan(&mut self, grid: &Grid, goal: &Grid, weight: u32, greedy: bool)
    {
        self.h = grid.linear_manhattan(goal) * weight;
        self.compute_f(greedy);
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
