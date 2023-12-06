fn main() {
    let time = 46807866;
    let distance: i64 = 214117714021024;
    let mut res = 0;
    for pressed_time in 1..time {
        let race = pressed_time * (time - pressed_time);
        if race > distance {
            res += 1;
        }
    }
    println!("{}", res);
}
