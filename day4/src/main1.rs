use regex::Regex;
use std::collections::HashSet;
pub(crate) use std::fs::read_to_string;
fn main() {
    let mut sum = 0;
    let re = Regex::new(r"\d+").unwrap();
    let input = read_to_string("input.txt").unwrap();
    for l in input.lines() {
        let mut hash: HashSet<&str> = HashSet::new();
        let res: u32 = (re
            .find_iter(&l[10..l.len()])
            .map(|x| hash.insert(x.as_str().trim()))
            .filter(|x| !*x)
            .count())
        .try_into()
        .unwrap();
        sum += i32::pow(2, res) / 2;
        println!("{:?} {:?} {} ", &l[10..l.len()], res, sum);
    }
    println!("{}", sum);
}
