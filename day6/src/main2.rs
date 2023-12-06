fn main() {
    let times: Vec<_> = vec![46, 80, 78, 66];

    let distances: Vec<_> = vec![214, 1177, 1402, 1024];

    let mut res: Vec<i32> = vec![0, 0, 0, 0];
    for (i, time) in times.iter().enumerate() {
        for pressed_time in 1..*time {
            let race = pressed_time * (time - pressed_time);
            if race > distances[i] {
                res[i] += 1;
            }
        }
    }
    println!("{:?}", res[0] * res[1] * res[2] * res[3]);
}
