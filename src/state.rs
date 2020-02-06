use std::cmp::{Ordering, Ord};

#[derive(Eq, PartialEq, PartialOrd)]
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