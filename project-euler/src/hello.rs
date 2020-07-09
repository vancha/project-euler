#[allow(dead_code)]
pub fn problem_001() {
    let mut numbers: Vec<i32> = (1..1000).collect(); //all numbers from one to 1000
    numbers.retain(|x| x % 3 == 0 || x % 5 == 0); //remove all of them that are not multiples of 3 and 5
    println!("{}", numbers.iter().sum::<i32>()); //print the sum of those numbers
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
    fn is_factor(number: i64, potentialfactor: i64) -> bool {
        if number % potentialfactor == 0 {
            true
        } else {
            false
        }
    }
    fn is_prime(input: i64) -> bool {
        match input <= 3 {
            true => {
                input > 1
            }
            false => match input % 2 == 0 || input % 3 == 0 {
                true => false,
                false => {
                    let mut i = 5;
                    loop {
                        if i * i > input {
                            break true;
                        }

                        if !(input % i == 0 || input % (i + 2) == 0) {
                            i = i + 6;
                            continue;
                        };
                        break false;
                    }
                }
            },
        }
    }
    /*let mut factors: Vec<i32> = vec![];
    let input = 600851475143f64;
    let rangeto: i64 = input.sqrt() as i64;
    for x in 1i64..input as i64 {
        //rangeto {
        if (is_factor(input as i64, x as i64)) {
            if (is_prime(x as i64)) {}
        }
    }*/
}
