use regex::Regex;
pub(crate) use std::fs::read_to_string;
fn main() {
    let mut sum = 0;

    let mut index: i32 = 1;
    let re: Regex = Regex::new(r"\d+ [grb]").unwrap();
    for line in read_to_string("input.txt").unwrap().lines() {
        let res = line.split(";").all(|s| {
            re.find_iter(s).all(|couple| {
                let split = couple.as_str().split_whitespace().collect::<Vec<_>>();
                let value = split[0].to_string().parse::<i32>().unwrap();
                match split[1] {
                    "g" => value < 14,
                    "b" => value < 15,
                    "r" => value < 13,
                    _ => false,
                }
            })
        });
        sum += (r * g * b);
    }
    println!("final sum {} ", sum);
}
