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
        //trial division algorithm with 6k+1 optimization, converted pseudocode from https://en.wikipedia.org/wiki/Primality_test
        match input <= 3 {
            true => input > 1,
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
    fn get_factors(input: i64) -> Vec<i32> {
        let mut a: Vec<i32> = vec![];
        let mut f = 2;
        let mut input = input;
        loop {
            if input <= 1 {
                break;
            };
            if input % f == 0 {
                a.push(f as i32);
                input /= f;
            } else {
                f += 1;
            }
        }
        a
    }
    let input = 600851475143f64 as i64;
    let factors = get_factors(input);
    //println!("factors of 9009: {:?}",get_factors(9009));
    println!("largest factors: {:?}", factors.iter().max());
}

#[allow(dead_code)]
pub fn problem_004() {
    fn is_palindrome(n1: i32) -> bool {
        let forward: Vec<char> = n1.to_string().chars().collect();
        let mut backward = forward.clone();
        backward.reverse();
        match forward == backward {
            true => true,
            false => false,
        }
    }
    let mut palindromes: Vec<i32> = vec![];
    for n1 in (100..1000).rev() {
        for n2 in (100..1000).rev() {
            if is_palindrome(n1 * n2) {
                palindromes.push(n1 * n2);
            }
        }
    }
    println!("largest is: {}", palindromes.iter().max().unwrap());
}

#[allow(dead_code)]
pub fn problem_005() {
    fn is_evenly_divisible(number: i32) -> bool {
        let numbers_to_divide_by: Vec<i32> = (1..20).collect();
        for x in numbers_to_divide_by {
            if number % x != 0 {
                 return false;
            }
        }
        true
    }
    for x in 1..900000000 {
    if is_evenly_divisible(x) {
        println!("{} works",x);
        break;
    }
    }
}


#[allow(dead_code)]
pub fn problem_006() {
    let mut sumofsquares:i32 = 0;
    let mut squareofsums:i32 = 0;
    for x in 1..101 {
        sumofsquares +=  (x as i32).pow(2);
        squareofsums += x;
    }
    squareofsums = squareofsums.pow(2);
    println!("{},{},{}",sumofsquares,squareofsums,(squareofsums - sumofsquares).abs());
}

