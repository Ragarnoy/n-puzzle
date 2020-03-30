use crate::{
    grid::{Grid, HType},
    node::Node
};
use std::{
    fmt,
    collections::{BinaryHeap, HashSet},
    rc::Rc,
    cell::RefCell,
};

pub struct Algo
{
    open_list: BinaryHeap<Rc<RefCell<Node>>>,
    closed_list: HashSet<Node>,
    path: Vec<Rc<RefCell<Node>>>,
    solution: Option<Rc<RefCell<Node>>>,
    goal: Grid,
    h_type: HType,
    a_type: AType,
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
        Self::IDAStar
    }
}

impl fmt::Display for AType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            Self::AStar => write!(f, "A*"),
            Self::IDAStar => write!(f, "IDA*")
        }
    }
}

impl AType
{
    pub fn from_str_or_default(input: Option<&str>) -> Result<Self, String>
    {
        match input
        {
            None => Ok(Self::default()),
            Some("astar") => Ok(Self::AStar),
            Some("idastar") => Ok(Self::IDAStar),
            Some(h) => Err(format!("This algorithmic function does not exist: {}", h))
        }
    }
}

impl Algo
{
    pub fn new(initial_node: Node, goal: Grid, h_type: HType, a_type: AType, min_weight: u32, max_weight: u32) -> Self
    {
        let initial_node = Rc::new(RefCell::new(initial_node));
        let mut open_list = BinaryHeap::new();
        open_list.push(Rc::clone(&initial_node));

        Algo
        {
            open_list,
            closed_list: HashSet::new(),
            path: vec![initial_node],
            solution: None,
            goal,
            h_type,
            a_type,
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

    pub fn get_total_cost_a_star(&self) -> u32
    {
        if let Some(sol) = self.solution.as_ref()
        {
            sol.borrow().state.g
        }
        else
        {
            0
        }
    }

    pub fn get_total_cost_ida_star(&self) -> u32
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

    pub fn get_total_cost(&self) -> u32
    {
        match self.a_type
        {
            AType::AStar => self.get_total_cost_a_star(),
            AType::IDAStar => self.get_total_cost_ida_star()
        }
    }

    pub fn get_weight(&self) -> u32
    {
        self.weight
    }

    pub fn print_steps(&self)
    {
        match self.a_type
        {
            AType::AStar => self.print_steps_a_star(),
            AType::IDAStar => self.print_steps_ida_star()
        }
    }

    pub fn print_steps_a_star(&self)
    {
        if let Some(sol) = self.solution.as_ref()
        {
            sol.borrow().print_steps();
        }
    }

    pub fn print_steps_ida_star(&self)
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
            c.borrow_mut().update_state(&self.goal, self.h_type, self.weight as u32);
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

    pub fn resolve_ida_star(&mut self) -> bool
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
                if threshold_change_count >= threshold_change_max && self.weight < self.max_weight
                {
                    self.weight += 1;
                    if self.weight % 5 == 0
                    {
                        threshold_change_max += 1;
                    }
                    threshold_change_count = 0;
                }
            }
        }
    }

    pub fn resolve_a_star(&mut self) -> bool
    {
        while let Some(node) = self.open_list.pop()
        {
            self.t_complex += 1;
            if node.borrow().state.h == 0 && node.borrow().grid == self.goal
            {
                self.solution = Some(node);
                return true;
            }
            self.closed_list.insert(node.borrow().clone());
            for child in Node::generate_childs(node)
            {
                // if self.closed_list.iter().any(|n| n.grid == child.borrow().grid)
                if self.closed_list.contains(&child.borrow())
                {
                    continue;
                }
                // Try to swap the two conditions below (and do thefor loop directly, no if around it)
                else if self.open_list.iter().any(|n| *n == child)
                {
                    if self.open_list.iter().any(|n| *n == child && n.borrow().state.g < child.borrow().state.g)
                    {
                        continue;
                    }
                    let child_g = child.borrow().state.g;
                    let child_parent = Rc::clone(child.borrow().parent.as_ref().unwrap());

                    for node in self.open_list.iter().filter(|&n| *n == child && n.borrow().state.g < child.borrow().state.g)
                    {
                        let new_f = node.borrow().state.h as u64 + child_g as u64;
                        node.borrow_mut().state.g = child_g;
                        node.borrow_mut().state.f = new_f;
                        node.borrow_mut().parent = Some(Rc::clone(&child_parent));
                    }
                }
                else
                {
                    child.borrow_mut().update_state(&self.goal, self.h_type, self.weight as u32);
                    if child.borrow().state.h == 0
                    {
                        self.solution = Some(child);
                        return true;
                    }
                    self.open_list.push(child)
                }
            }
            let max_states = self.open_list.len() + self.closed_list.len();
            if self.s_complex < max_states as u64
            {
                self.s_complex = max_states as u64;
            }
        }
        false
    }

    pub fn resolve(&mut self) -> bool
    {
        match self.a_type
        {
            AType::AStar => self.resolve_a_star(),
            AType::IDAStar => self.resolve_ida_star()
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