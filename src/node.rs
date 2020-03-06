use std::cmp::{Ordering, Ord};
use std::rc::Rc;
use std::cell::RefCell;
use crate::{state::State, grid::{Grid, HType}};
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug)]
pub struct Node
{
    pub grid: Grid,
    pub state: State,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Hash for Node
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.grid.hash(state);
    }
}

impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool
    {
        self.grid == other.grid
    }
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
    pub fn new(state: State, grid: Grid) -> Self
    {
        Node
        {
            state,
            grid,
            parent: None,
        }
    }

    pub fn generate_childs(node: Rc<RefCell<Self>>) -> Vec<Rc<RefCell<Node>>>
    {
        let mut ret:Vec<Rc<RefCell<Node>>> = Vec::new();
        for grid in node.borrow().grid.move_all_possible()
        {
            ret.push(Rc::new(RefCell::new(Node
            {
                grid,
                state: State::new(0, node.borrow().state.g + 1, 0),
                parent: Some(Rc::clone(&node)),
            })));
        }
        ret
    }

    pub fn update_state(&mut self, goal: &Grid, h_type: HType, weight: u32)
    {
        match h_type
        {
            HType::Hamming => self.state.update_hamming(&self.grid, goal, weight),
            HType::Manhattan => self.state.update_manhattan(&self.grid, goal, weight),
            HType::LinearManhattan => self.state.update_linear_manhattan(&self.grid, goal, weight)
        }
    }

    pub fn print_steps(&self)
    {
        if let Some(parent) = &self.parent
        {
            parent.borrow().print_steps();
        }
        println!("{}", self.grid);
        println!("===================================\n");
    }
}
