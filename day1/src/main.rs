pub(crate) use std::fs::read_to_string;

fn main() {
    let mut sum  = 0;
    const  RADIX: u32 = 10;
    for line in read_to_string("input.txt").unwrap().lines() {
        let res  = line.to_string().chars().filter(|s| s.is_numeric()).collect::<Vec<char>>();
        sum += res[0].to_digit(RADIX).unwrap()  * 10 + res[res.len() - 1].to_digit(RADIX).unwrap(); 
    }
    println!("{}",sum);
}