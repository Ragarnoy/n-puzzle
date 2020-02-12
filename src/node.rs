use std::cmp::{Ordering, Ord};
use std::rc::Rc;
use crate::{state::State, grid::Grid};

#[derive(Eq, PartialEq, Clone)]
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

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.state.cmp(&other.state))
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

    fn generate_child(rcnode: Rc<Self>, col: u8) -> Vec<Node>
    {
        let mut ret:Vec<Node> = Vec::new();
        for grid in rcnode.grid.move_all_possible(col)
        {
            ret.push(Node
            {
                grid,
                state: State::new(0, rcnode.state.g, 0),
                parent: Some(Rc::clone(&rcnode)),
            }
            );
        }
        ret
    }


}
