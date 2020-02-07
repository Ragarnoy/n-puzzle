use std::cmp::{Ordering, Ord};
#[derive(Eq, PartialEq, PartialOrd, Clone)]
pub struct State
{
    h:u16,
    g:u32,
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