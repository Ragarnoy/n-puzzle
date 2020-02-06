use std::cmp::{Ordering, Ord};
use crate::{state::State, grid::Grid};

#[derive(Eq, PartialEq, PartialOrd)]
pub struct Node
{
    state: State,
    grid: Grid,
    parent: Option<Box<Node>>,
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.state.cmp(&other.state)
    }
}

