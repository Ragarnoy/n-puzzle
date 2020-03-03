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
    open_list: BinaryHeap<Rc<RefCell<Node>>>,
    closed_list : Vec<Rc<RefCell<Node>>>,
    goal: Grid,
    column: u8,
    h_type: HType
}

impl Algo
{
    pub fn new(initial_node: Node, goal: Grid, h_type: HType, column: u8) -> Self
    {
        let mut open_list: BinaryHeap<Rc<RefCell<Node>>> = BinaryHeap::new();
        open_list.push(Rc::new(RefCell::new(initial_node)));

        Algo
        {
            open_list,
            closed_list: Vec::new(),
            goal,
            column,
            h_type
        }
    }

    pub fn resolve(&mut self) -> Option<Rc<RefCell<Node>>>
    {
        while let Some(node) = self.open_list.pop()
        {
            if node.borrow().state.h == 0
            {
                return Some(node);
            }
            self.closed_list.push(Rc::clone(&node));
            for child in Node::generate_childs(node, self.column)
            {
                if self.closed_list.iter().any(|n| n.borrow().grid == child.borrow().grid)
                {
                    continue;
                }
                else if self.open_list.iter().any(|n| *n == child && n.borrow().state.g < child.borrow().state.g)
                {
                    let child_g = child.borrow().state.g;
                    let child_parent = Rc::clone(child.borrow().parent.as_ref().unwrap());

                    for node in self.open_list.iter().filter(|&n| *n == child && n.borrow().state.g < child.borrow().state.g)
                    {
                        let new_f = node.borrow().state.h as u32 + child_g;
                        node.borrow_mut().state.g = child_g;
                        node.borrow_mut().state.f = new_f;
                        node.borrow_mut().parent = Some(Rc::clone(&child_parent));
                    }
                }
                else
                {
                    // println!("DEBUG::algo::resolve::child: {:#?}", child);
                    child.borrow_mut().update_state(&self.goal, self.h_type, self.column);
                    if child.borrow().state.h == 0 {
                        return Some(child);
                    }
                    self.open_list.push(child)
                }
            }
        }
        None
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