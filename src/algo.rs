use crate::{
    grid::{Grid, HType},
    node::Node
};
use std::{
    collections::BinaryHeap,
    rc::Rc,
    cell::RefCell,
    time::{Duration, Instant},
    sync::{Arc, Mutex}
};

pub struct Algo
{
    // initial_node: Node,
    path: Vec<Rc<RefCell<Node>>>,
    goal: Grid,
    h_type: HType,
    t_complex: u64,
    s_complex: u64,
    weight: u32,
    other: Option<Arc<Mutex<Self>>>
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
    pub fn new(initial_node: Node, goal: Grid, h_type: HType, other: Option<Arc<Mutex<Self>>>) -> Self
    {
        let weight = 1;
        // let weight = initial_node.state.h as u32 / 2;

        Algo
        {
            path: vec![Rc::new(RefCell::new(initial_node))],
            // initial_node,
            goal,
            h_type,
            t_complex: 0,
            s_complex: 0,
            weight,
            other
        }
    }

    pub fn get_s_complex(&self) -> u64
    {
        self.s_complex
    }

    pub fn get_t_complex(&self) -> u64
    {
        self.t_complex
    }

    pub fn set_other(&mut self, other: Option<Arc<Mutex<Self>>>)
    {
        self.other = other;
    }

    // fn explore_node(&mut self, path: &mut Vec<Rc<RefCell<Node>>>, threshold: u32) -> u32
    fn explore_node(&mut self, threshold: u32) -> u32
    {
        if self.path.last().is_none()
        {
            return u32::max_value();
        }

        self.t_complex += 1;
        let node = self.path.last().unwrap();
        let curr_f = node.borrow().state.f;

        if curr_f as u32 > threshold
        {
            return curr_f;
        }
        else if node.borrow().state.h == 0
        {
            return 0;
        }
        else if let Some(other) = self.other
        {
            let other = other.lock().unwrap();
            if other.path.contains(node)
            {
                return 0;
            }
        }
        let mut lowest_f = u32::max_value();
        let childs: BinaryHeap<Rc<RefCell<Node>>> = Node::generate_childs(Rc::clone(node)).into_iter().map(|c| {
            let update_time = Instant::now();
            c.borrow_mut().update_state(&self.goal, self.h_type, self.weight);
            // println!("DEBUG::algo::explore_node: Time to update state of child: {:#?}", update_time.elapsed());
            Rc::clone(&c)
        }).collect();
        let s_complex = self.path.len() as u64 + childs.len() as u64;
        if s_complex > self.s_complex
        {
            self.s_complex = s_complex;
        }
        for child in childs
        {
            if !self.path.contains(&child)
            {
                self.path.push(Rc::clone(&child));
                // let recurs_res = self.explore_node(self.path, threshold);
                let recurs_res = self.explore_node(threshold);
                if recurs_res == 0
                {
                    return 0;
                }
                else if recurs_res < lowest_f
                {
                    lowest_f = recurs_res;
                }
                self.path.pop();
            }
        }
        return lowest_f;
    }

    pub fn resolve(&mut self) -> Vec<Rc<RefCell<Node>>>
    {
        let mut threshold = self.path.last().unwrap().borrow().state.h as u32;
        // let mut path = vec![Rc::new(RefCell::new(self.initial_node.clone()))];
        let max_weight = 10;
        self.s_complex += 1;

        loop
        {
            // let recurs_res = self.explore_node(&mut path, threshold);
            let recurs_res = self.explore_node(threshold);
            if recurs_res == 0
            {
                return self.path;
            }
            else if recurs_res == u32::max_value()
            {
                return Vec::new();
            }
            else
            {
                threshold = recurs_res;
                println!("New threshold: {}", threshold);
                if self.weight + 1 < max_weight
                {
                    self.weight += 1;
                }
                println!("New weight: {}", self.weight);
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