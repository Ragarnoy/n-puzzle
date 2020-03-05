use crate::{
    grid::{Grid, HType},
    node::Node
};
use std::{
    collections::BinaryHeap,
    rc::Rc,
    cell::RefCell
};

pub struct Algo
{
    initial_node: Node,
    goal: Grid,
    column: u8,
    h_type: HType,
}

#[derive(Copy, Clone, Debug)]
pub enum AType
{
    AStar,
    IDAStar
}

impl Default for AType
{
    fn default() -> Self
    {
        Self::AStar
    }
}

impl AType
{
    pub fn from_str_or_default(input: Option<&str>) -> Result<Self, String>
    {
        match input
        {
            None => Ok(Self::default()),
            Some("idastar") => Ok(Self::AStar),
            Some("astar") => Ok(Self::IDAStar),
            Some(h) => Err(format!("This algorithmic function does not exist: {}", h))
        }
    }
}

impl Algo
{
    pub fn new(initial_node: Node, goal: Grid, h_type: HType, column: u8) -> Self
    {
        Algo
        {
            initial_node,
            goal,
            column,
            h_type
        }
    }

    fn explore_node(&mut self, path: &mut Vec<Rc<RefCell<Node>>>, threshold: u32) -> u32
    {
        if path.last().is_none()
        {
            return u32::max_value();
        }

        let node = path.last().unwrap();
        let curr_f = node.borrow().state.f;

        if curr_f as u32 > threshold
        {
            return curr_f;
        }
        else if node.borrow().state.h == 0
        {
            return 0;
        }
        let mut lowest_f = u32::max_value();
        let childs: BinaryHeap<Rc<RefCell<Node>>> = Node::generate_childs(Rc::clone(node)).into_iter().map(|c| {
            c.borrow_mut().update_state(&self.goal, self.h_type);
            Rc::clone(&c)
        }).collect();
        for child in childs
        {
            if !path.contains(&child)
            {
                path.push(Rc::clone(&child));
                let recurs_res = self.explore_node(path, threshold);
                if recurs_res == 0
                {
                    return 0;
                }
                else if recurs_res < lowest_f
                {
                    lowest_f = recurs_res;
                }
                path.pop();
            }
        }
        return lowest_f;
    }

    pub fn resolve(&mut self) -> Vec<Rc<RefCell<Node>>>
    {
        let mut threshold = self.initial_node.state.h as u32;
        let mut path = vec![Rc::new(RefCell::new(self.initial_node.clone()))];

        loop
        {
            let recurs_res = self.explore_node(&mut path, threshold);
            if recurs_res == 0
            {
                return path;
            }
            else if recurs_res == u32::max_value()
            {
                return Vec::new();
            }
            else
            {
                threshold = recurs_res;
            }
        }
    }

}

#[cfg(test)]
mod tests
{
    use crate::state::State;
    use std::{collections::{BinaryHeap, BTreeSet}, rc::Rc};

    #[test]
    fn test_binary_heap_sort()
    {
        let mut bh: BinaryHeap<Rc<State>> = BinaryHeap::new();
        let s0 = Rc::new(State::new(0, 0, 3));
        let s1 = Rc::new(State::new(0, 0, 6));
        let s2 = Rc::new(State::new(0, 1, 6));

        bh.push(Rc::clone(&s2));
        bh.push(Rc::clone(&s0));
        bh.push(Rc::clone(&s1));

        assert_eq!(s0, bh.pop().unwrap());
        assert_eq!(s1, bh.pop().unwrap());
        assert_eq!(s2, bh.pop().unwrap());
        assert_eq!(None, bh.pop());
    }

    #[test]
    fn test_btree_set_storage()
    {
        let mut bts: BTreeSet<Rc<State>> = BTreeSet::new();
        let b0 = Rc::new(State::new(0, 1, 1));
        let b1 = Rc::new(State::new(1, 0, 1));
        let b2 = Rc::new(State::new(0, 0, 0));
        let b3 = Rc::new(State::new(0, 1, 1));

        bts.insert(Rc::clone(&b0));
        assert_eq!(1, bts.len());
        bts.insert(Rc::clone(&b1));
        assert_eq!(2, bts.len());
        bts.insert(Rc::clone(&b2));
        assert_eq!(3, bts.len());
        bts.insert(Rc::clone(&b3));
        assert_eq!(3, bts.len());
    }
}