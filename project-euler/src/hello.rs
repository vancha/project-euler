#[allow(dead_code)]
pub fn problem_001() {
    let mut numbers: Vec<i32> = (1..999).collect();
    numbers.retain(|x| x % 3 == 0 || x % 5 == 0);
    println!("{}", numbers.iter().sum::<i32>());
}

#[allow(dead_code)]
pub fn problem_002() {
    let mut a = 1;
    let mut b = 2;
    let mut sum: Vec<i32> = vec![2];
    loop {
        if a + b < 4000000 {
            let (ta, tb) = (a, b);
            b = tb + ta;
            a = tb;
            if b % 2 == 0 {
                sum.push(b);
            }
        } else {
            break;
        }
    }
    println!("sum: {:?}", sum.iter().sum::<i32>());
}

#[allow(dead_code)]
pub fn problem_003() {
    fn is_prime() -> bool {
        true
    }
    //println!("test");
}
