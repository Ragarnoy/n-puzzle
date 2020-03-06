extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use utils::snail_sort;

pub fn create_snail_goal(lgth: u8) -> Vec<u16>
{
    let mut ret: Vec<u16> = (1..lgth as u16 * lgth as u16).collect();
    ret.push(0);
    snail_sort(&ret, lgth)
}