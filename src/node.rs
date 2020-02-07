use std::cmp::{Ordering, Ord};
use std::rc::Rc;
use crate::{state::State, grid::Grid};

#[derive(Eq, PartialEq, PartialOrd, Clone)]
pub struct Node
{
    grid: Grid,
    state: State,
    parent: Option<Rc<Node>>,
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.state.cmp(&other.state)
    }
}

impl Node
{
    fn new(state: State, grid: Grid) -> Self
    {
        Node
        {
            state,
            grid,
            parent: None,
        }
    }

    fn generate_child(rcnode: Rc<Self>) -> Vec<Node>
    {
        let mut ret:Vec<Node> = Vec::new();
        for grid in //Grid::Move()
        {
            ret.push(Node
            {
                grid, // grid
                state, //state from grid
                parent: Some(rcnode), //?
            }
            );
        }
        ret
    }


}