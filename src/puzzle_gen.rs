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

pub fn random_puzzle(input: u8) -> Vec<u16>
{
    let mut rng = thread_rng();
    let mut snail: Vec<u16> = (0..input as u16 * input as u16).collect();
    snail.shuffle(&mut rng);
    snail
}

