use regex::Regex;
use std::collections::HashSet;

pub(crate) use std::fs::read_to_string;
fn main() {
    //let mut sum = 0;
    let re = Regex::new(r"\d+").unwrap();
    let input = read_to_string("input.txt").unwrap();

    let mut map: Vec<_> = vec![];
    let mut hash: HashSet<&str> = HashSet::new();
    for (i, l) in input.lines().enumerate() {
        let res = re
            .find_iter(&l[10..l.len()])
            .filter(|x| !hash.insert(x.as_str().trim()))
            .count();

        map.push(i + 1..i + res);

        /*println!(
            "{:?}",
            map.pop()
                .unwrap()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );*/
    }

    for i in (1..map.len()).rev() {
        println!("{} {:?}", i, &map[i - 1]);
    }
}
