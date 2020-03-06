use crate::{
    grid::{Grid, HType},
    node::Node
};
use std::{
    collections::BinaryHeap,
    rc::Rc,
    cell::RefCell,
};

pub struct Algo
{
    path: Vec<Rc<RefCell<Node>>>,
    goal: Grid,
    h_type: HType,
    t_complex: u64,
    s_complex: u64,
    weight: u32,
    max_weight: u32
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
    pub fn new(initial_node: Node, goal: Grid, h_type: HType, max_weight: u32, min_weight: u32) -> Self
    {
        Algo
        {
            path: vec![Rc::new(RefCell::new(initial_node))],
            goal,
            h_type,
            t_complex: 0,
            s_complex: 0,
            weight: min_weight,
            max_weight
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

    pub fn get_total_cost(&self) -> u32
    {
        if let Some(node) = self.path.last()
        {
            node.borrow().state.g
        }
        else
        {
            0
        }
    }

    pub fn print_steps(&self)
    {
        if !self.path.is_empty()
        {
            for node in self.path.iter()
            {
                println!("{}", node.borrow().grid);
                println!("===================================\n");
            }   
        }
    }

    fn explore_node(&mut self, threshold: u64) -> u64
    {
        if self.path.last().is_none()
        {
            return u64::max_value();
        }

        self.t_complex += 1;
        let node = self.path.last().unwrap();
        let curr_f = node.borrow().state.f;

        if curr_f > threshold
        {
            return curr_f;
        }
        else if node.borrow().state.h == 0
        {
            return 0;
        }
        let mut lowest_f = u64::max_value();
        let childs: BinaryHeap<Rc<RefCell<Node>>> = Node::generate_childs(Rc::clone(node)).into_iter().map(|c| {
            c.borrow_mut().update_state(&self.goal, self.h_type, self.weight);
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

    pub fn resolve(&mut self) -> bool
    {
        let mut threshold = self.path.last().unwrap().borrow().state.h as u64;
        let mut threshold_change_count = 0;
        let mut threshold_change_max = 1;
        self.s_complex += 1;

        loop
        {
            let recurs_res = self.explore_node(threshold);
            if recurs_res == 0
            {
                return true;
            }
            else if recurs_res == u64::max_value()
            {
                return false;
            }
            else
            {
                threshold = recurs_res;
                threshold_change_count += 1;
                println!("New threshold: {}", threshold);
                if threshold_change_count >= threshold_change_max && self.weight < self.max_weight
                {
                    self.weight += 1;
                    if self.weight % 5 == 0
                    {
                        threshold_change_max += 1;
                    }
                    threshold_change_count = 0;
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