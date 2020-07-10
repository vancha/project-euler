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

#[allow(dead_code)]
pub fn problem_003() {
    fn is_factor(number: i64, potentialfactor: i64) -> bool {
        if number % potentialfactor == 0 {
            true
        } else {
            false
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
            println!("{} works", x);
            break;
        }
    }
}

#[allow(dead_code)]
pub fn problem_006() {
    let mut sumofsquares: i32 = 0;
    let mut squareofsums: i32 = 0;
    for x in 1..101 {
        sumofsquares += (x as i32).pow(2);
        squareofsums += x;
    }
    squareofsums = squareofsums.pow(2);
    println!(
        "{},{},{}",
        sumofsquares,
        squareofsums,
        (squareofsums - sumofsquares).abs()
    );
}

#[allow(dead_code)]
pub fn problem_007() {
    let mut primes: Vec<i64> = vec![];
    let mut start = 0;
    while primes.len() != 10001 {
        for x in start..90000000000 {
            if is_prime(x) { 
                primes.push(x as i64);
                start = x + 1;
                break;
            }
        }
    }
    println!("10001st prime: {:?}", primes.last());
}


#[allow(dead_code)]
pub fn problem_008() {
    
    let problem_data = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let mut max = 0;
    let mut substring = "";
    for x in 0..(problem_data.len()+1 -13) { //loop as often as there are characters
        let slice = &problem_data[x..x+13];
        let mut product: i64 = 1;
        for c in slice.chars() {
            product = product * c.to_digit(10).unwrap() as i64; 
        }
        if product > max {
            max = product;
            substring = slice;
        } 
    }
    println!("{}",max);

}
