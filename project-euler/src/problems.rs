    /// Multiples of 3 and 5
    ///
    /// # Description
    ///
    ///If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    ///Find the sum of all the multiples of 3 or 5 below 1000.
#[allow(dead_code)]
pub fn problem_001()-> i32 {
    let mut numbers: Vec<i32> = (1..1000).collect(); 
    numbers.retain(|x| x % 3 == 0 || x % 5 == 0); //remove all of them that are not multiples of 3 and 5
    //println!("{}", numbers.iter().sum::<i32>()); //print the sum of those numbers
    numbers.iter().sum::<i32>()
}

/// Even Fibonacci numbers
///
/// # Description 
///
///Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
///1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
///
#[allow(dead_code)]
pub fn problem_002() {
    //the "terms" a and b
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

///Helper method to check if a number is prime
///
/// # Description
///
///trial division algorithm with 6k+1 optimization, converted pseudocode from https://en.wikipedia.org/wiki/Primality_test
fn is_prime(input: i64) -> bool {
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

/// Largest prime factor
///
///# Description
///
///The prime factors of 13195 are 5, 7, 13 and 29.
///What is the largest prime factor of the number 600851475143 ?
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

/// Largest palindrome product
///
/// # Description
///
///A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///Find the largest palindrome made from the product of two 3-digit numbers.
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
/// Smallest multiple
///
/// # Description
///
///2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

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

///Sum square difference
///
///# Description
///
///The sum of the squares of the first ten natural numbers is 12+22+...+102=385. 
///The square of the sum of the first ten natural numbers is (1+2+...+10)2=552=3025.
///Hence,  the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=2640.
///Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
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

/// 10001st prime
///
/// # Description
///
///By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///What is the 10 001st prime number?
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

///Largest product in a series
///
/// # Description
///
///The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
///
///73167176531330624919225119674426574742355349194934
///96983520312774506326239578318016984801869478851843
///85861560789112949495459501737958331952853208805511
///12540698747158523863050715693290963295227443043557
///66896648950445244523161731856403098711121722383113
///62229893423380308135336276614282806444486645238749
///30358907296290491560440772390713810515859307960866
///70172427121883998797908792274921901699720888093776
///65727333001053367881220235421809751254540594752243
///52584907711670556013604839586446706324415722155397
///53697817977846174064955149290862569321978468622482
///83972241375657056057490261407972968652414535100474
///82166370484403199890008895243450658541227588666881
///16427171479924442928230863465674813919123162824586
///17866458359124566529476545682848912883142607690042
///24219022671055626321111109370544217506941658960408
///07198403850962455444362981230987879927244284909188
///84580156166097919133875499200524063689912560717606
///05886116467109405077541002256983155200055935729725
///71636269561882670428252483600823257530420752963450
///
///
///Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
#[allow(dead_code)]
pub fn problem_008() {
    let problem_data = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let mut max = 0;
    for x in 0..(problem_data.len() + 1 - 13) {
        //loop as often as there are characters
        let slice = &problem_data[x..x + 13];
        let mut product: i64 = 1;
        for c in slice.chars() {
            product = product * c.to_digit(10).unwrap() as i64;
        }
        if product > max {
            max = product;
        }
    }
    println!("{}", max);
}

/// Special Pythagorean triplet
///
///# Description
///
///A Pythagorean triplet is a set of three natural numbers, a < b < c, for which a2 + b2 = c2
///
///For example, 32 + 42 = 9 + 16 = 25 = 52.
///
///There exists exactly one Pythagorean triplet for which a + b + c = 1000.
///Find the product abc.
#[allow(dead_code)]
pub fn problem_009() {
    for a in 0..1000 {
        for b in 0..1000 {
            for c in 0..1000 {
                if a < b && b < c {
                    if (a as i32).pow(2) + (b as i32).pow(2) == (c as i32).pow(2) {
                        if a + b + c == 1000 {
                            println!(" seem to have gotten the answer: {} + {} + {} == 1000, product = {}",a,b,c,(a * b * c));
                        }
                    }
                }
            }
        }
    }
}


///Summation of primes
///
/// # Description
///
///The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
///Find the sum of all the primes below two million.

#[allow(dead_code)]
pub fn problem_010() {
    let mut sum: i64 = 0;
 for x in 0..2000000 {
     if is_prime(x) {
        sum += x;
     } 
 }
 println!("sum of all primes below 2 million: {}",sum);
}

///Largest product in a grid
///
/// # Description
///
///In the 20×20 grid below, four numbers along a diagonal line have been marked in red.
///
///
///08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08  
///49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00  
///81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65  
///52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91  
///22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80  
///24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50  
///32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70  
///67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21  
///24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72  
///21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95  
///78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92  
///16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57  
///86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58  
///19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40  
///04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66  
///88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69  
///04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36  
///20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16  
///20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54  
///01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48  
///
///The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
///
///What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
#[allow(dead_code)]
pub fn problem_011() {
    /*  "01 02 03 99 48".split(" ").map(str::parse::<i32>).collect::<Result<Vec<_>, _>>() */   
    struct EulerMatrix {
        data: Vec<Vec<i32>>,
        position: (i32,i32),
    }
        impl EulerMatrix {
            fn new(data: Vec<Vec<i32>>) -> Self {
               EulerMatrix { 
                    data: data,
                    position: (0,0),
                }
            }
        }
        impl Iterator for EulerMatrix {
            type Item = i32;
            
            fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> { 
                
                let (x,y) = self.position;


                if x >= 20 {
                    self.position.0 = 0;
                    self.position.1 += 1;
                } else {
                    self.position.0 += 1;
                }
                
                
                let (x,y) = (x as usize,y as usize);
                let horizontal = vec![self.data.get(y).unwrap().get(x), self.data.get(y).unwrap().get(x+1), self.data.get(y).unwrap().get(x+2), self.data.get(y).unwrap().get(x+3)];
                let vertical = vec![
                    match self.data.get(y) { 
                        Some(val) => {
                            val.get(x)
                        },
                        None => None,
                },
                match self.data.get(y+1) {
                    Some(val) => {
                        val.get(x)
                    },
                    None => None,
                },
                match self.data.get(y+2) {
                    Some(val) => {
                        val.get(x)
                    }
                    None => None,
                },
                match self.data.get(y+3) {
                    Some(val) => {
                    val.get(x)
                    }
                    None => None,
                }];

                let diagonal_tl_to_br = vec![
                    match self.data.get(y) {
                        Some(val) => {
                            val.get(x)
                        },
                        None => None,
                },
                match self.data.get(y+1) {
                    Some(val) => {
                        val.get(x+1)
                    },
                    None => None,
                },
                match self.data.get(y+2) {
                    Some(val) => {
                        val.get(x+2)
                    }
                    None => None,
                },
                match self.data.get(y+3) {
                    Some(val) => {
                    val.get(x+3)
                    }
                    None => None,
                }];

                let diagonal_tr_to_bl = vec![
                    match self.data.get(y) {
                        Some(val) => {
                            val.get(x)
                        },
                        None => None,
                },
                match self.data.get(y+1) {
                    Some(val) => {
                        val.get((x as i32 -1) as usize)
                    },
                    None => None,
                },
                match self.data.get(y+2) {
                    Some(val) => {
                        val.get((x as i32-2) as usize)
                    }
                    None => None,
                },
                match self.data.get(y+3) {
                    Some(val) => {
                    val.get((x as i32 -3) as usize)
                    }
                    None => None,
                }];
                
                
               // println!("{:?}",horizontal);//diagonal_tr_to_bl);
                let result:Vec<i32> = vec![
                    match horizontal.contains(&None) {
                        true=> 0,
                        false => horizontal.iter().map(|x| x.unwrap()).product(),//println!("aaw yee all filled in"),
                    
                    },

                match vertical.contains(&None) {
                    true=> 0,
                    false => vertical.iter().map(|x| x.unwrap()).product(),//println!("aaw yee all filled in"),

                },
                match diagonal_tl_to_br.contains(&None) {
                    true => 0,
                    false => diagonal_tl_to_br.iter().map(|x| x.unwrap()).product(),//println!("aaw yee all filled in"),
                },

                match diagonal_tr_to_bl.contains(&None) {
                     true => 0,
                     false => diagonal_tr_to_bl.iter().map(|x| x.unwrap()).product(),//println!("aaw yee all filled in"),
                 }];
                let maxproduct = result.iter().max().unwrap();
                

                //println!("product of horizontal = {:?}",maxproduct);
                match self.position.1 >= 20 {
                    true => { 
                        None
                    },
                    false => {
                        Some(*maxproduct)
                    },
                } 
            }
        }
        let matrix = EulerMatrix::new(vec![
vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
vec![16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
vec![86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
vec![19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
vec![04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
vec![88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
vec![04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
vec![20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
vec![01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48]
]);
        
        let max = matrix.into_iter().max();
        println!("{}",max.unwrap());
}

///Highly divisible triangular number
///
/// # Description
///
///The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
///1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
///Let us list the factors of the first seven triangle numbers:
///
///     1: 1
///     3: 1,3
///     6: 1,2,3,6
///    10: 1,2,5,10
///    15: 1,3,5,15
///    21: 1,3,7,21
///    28: 1,2,4,7,14,28
///
///We can see that 28 is the first triangle number to have over five divisors.
///
///What is the value of the first triangle number to have over five hundred divisors?
#[allow(dead_code)]
pub fn problem_012() {
    for input in 1.. { 
        let triangle_number: i32 = (0..input+1).into_iter().sum();
        let nr_of_divisors : i32 = (1..(triangle_number as f64).sqrt() as i32).filter(|x| triangle_number % x == 0).count() as i32 * 2;//THIS DOES NOT RETURN THE CORRECT NUMBER ALL OF THE TIME!!
        if nr_of_divisors >= 500 {
            println!("{} has {} divisors",triangle_number,nr_of_divisors);
            break;
        }
     }
}
///Large sum
/*
Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
*/
#[allow(dead_code)]
pub fn problem_013() {
    let mut input :Vec<String> = vec![
        "37107287533902102798797998220837590246510135740250".into(),
"46376937677490009712648124896970078050417018260538".into(),
"74324986199524741059474233309513058123726617309629".into(),
"91942213363574161572522430563301811072406154908250".into(),
"23067588207539346171171980310421047513778063246676".into(),
"89261670696623633820136378418383684178734361726757".into(),
"28112879812849979408065481931592621691275889832738".into(),
"44274228917432520321923589422876796487670272189318".into(),
"47451445736001306439091167216856844588711603153276".into(),
"70386486105843025439939619828917593665686757934951".into(),
"62176457141856560629502157223196586755079324193331".into(),
"64906352462741904929101432445813822663347944758178".into(),
"92575867718337217661963751590579239728245598838407".into(),
"58203565325359399008402633568948830189458628227828".into(),
"80181199384826282014278194139940567587151170094390".into(),
"35398664372827112653829987240784473053190104293586".into(),
"86515506006295864861532075273371959191420517255829".into(),
"71693888707715466499115593487603532921714970056938".into(),
"54370070576826684624621495650076471787294438377604".into(),
"53282654108756828443191190634694037855217779295145".into(),
"36123272525000296071075082563815656710885258350721".into(),
"45876576172410976447339110607218265236877223636045".into(),
"17423706905851860660448207621209813287860733969412".into(),
"81142660418086830619328460811191061556940512689692".into(),
"51934325451728388641918047049293215058642563049483".into(),
"62467221648435076201727918039944693004732956340691".into(),
"15732444386908125794514089057706229429197107928209".into(),
"55037687525678773091862540744969844508330393682126".into(),
"18336384825330154686196124348767681297534375946515".into(),
"80386287592878490201521685554828717201219257766954".into(),
"78182833757993103614740356856449095527097864797581".into(),
"16726320100436897842553539920931837441497806860984".into(),
"48403098129077791799088218795327364475675590848030".into(),
"87086987551392711854517078544161852424320693150332".into(),
"59959406895756536782107074926966537676326235447210".into(),
"69793950679652694742597709739166693763042633987085".into(),
"41052684708299085211399427365734116182760315001271".into(),
"65378607361501080857009149939512557028198746004375".into(),
"35829035317434717326932123578154982629742552737307".into(),
"94953759765105305946966067683156574377167401875275".into(),
"88902802571733229619176668713819931811048770190271".into(),
"25267680276078003013678680992525463401061632866526".into(),
"36270218540497705585629946580636237993140746255962".into(),
"24074486908231174977792365466257246923322810917141".into(),
"91430288197103288597806669760892938638285025333403".into(),
"34413065578016127815921815005561868836468420090470".into(),
"23053081172816430487623791969842487255036638784583".into(),
"11487696932154902810424020138335124462181441773470".into(),
"63783299490636259666498587618221225225512486764533".into(),
"67720186971698544312419572409913959008952310058822".into(),
"95548255300263520781532296796249481641953868218774".into(),
"76085327132285723110424803456124867697064507995236".into(),
"37774242535411291684276865538926205024910326572967".into(),
"23701913275725675285653248258265463092207058596522".into(),
"29798860272258331913126375147341994889534765745501".into(),
"18495701454879288984856827726077713721403798879715".into(),
"38298203783031473527721580348144513491373226651381".into(),
"34829543829199918180278916522431027392251122869539".into(),
"40957953066405232632538044100059654939159879593635".into(),
"29746152185502371307642255121183693803580388584903".into(),
"41698116222072977186158236678424689157993532961922".into(),
"62467957194401269043877107275048102390895523597457".into(),
"23189706772547915061505504953922979530901129967519".into(),
"86188088225875314529584099251203829009407770775672".into(),
"11306739708304724483816533873502340845647058077308".into(),
"82959174767140363198008187129011875491310547126581".into(),
"97623331044818386269515456334926366572897563400500".into(),
"42846280183517070527831839425882145521227251250327".into(),
"55121603546981200581762165212827652751691296897789".into(),
"32238195734329339946437501907836945765883352399886".into(),
"75506164965184775180738168837861091527357929701337".into(),
"62177842752192623401942399639168044983993173312731".into(),
"32924185707147349566916674687634660915035914677504".into(),
"99518671430235219628894890102423325116913619626622".into(),
"73267460800591547471830798392868535206946944540724".into(),
"76841822524674417161514036427982273348055556214818".into(),
"97142617910342598647204516893989422179826088076852".into(),
"87783646182799346313767754307809363333018982642090".into(),
"10848802521674670883215120185883543223812876952786".into(),
"71329612474782464538636993009049310363619763878039".into(),
"62184073572399794223406235393808339651327408011116".into(),
"66627891981488087797941876876144230030984490851411".into(),
"60661826293682836764744779239180335110989069790714".into(),
"85786944089552990653640447425576083659976645795096".into(),
"66024396409905389607120198219976047599490197230297".into(),
"64913982680032973156037120041377903785566085089252".into(),
"16730939319872750275468906903707539413042652315011".into(),
"94809377245048795150954100921645863754710598436791".into(),
"78639167021187492431995700641917969777599028300699".into(),
"15368713711936614952811305876380278410754449733078".into(),
"40789923115535562561142322423255033685442488917353".into(),
"44889911501440648020369068063960672322193204149535".into(),
"41503128880339536053299340368006977710650566631954".into(),
"81234880673210146739058568557934581403627822703280".into(),
"82616570773948327592232845941706525094512325230608".into(),
"22918802058777319719839450180888072429661980811197".into(),
"77158542502016545090413245809786882778948721859617".into(),
"72107838435069186155435662884062257473692284509516".into(),
"20849603980134001723930671666823555245252804609722".into(),
"53503534226472524250874054075591789781264330331690".into()];
    
    //answer to append digits to when calculating
    let mut answer:Vec<i32> = vec![];
    //carry is outside the loop, will be calculated and cleared every iteration
    let mut carry = 0;
    //let xu = input.clone();
    for digit_to_process in 0..input[0].len() {
        //creates a temporary sum, to put the sum of every row of digits in
        let mut subsum: u128 = 0;
        //loops over all the digits in the numbers vertically
        for row_in_input in 0..input.len() {
            //gets the digit as a char, turns it into a u128
            let vertical_digit = (input[row_in_input].chars().rev().collect::<String>().get(digit_to_process..digit_to_process+1)).unwrap().parse::<u128>().unwrap();
            //ads this digit to the temporary sum
            subsum += vertical_digit;  
        }
        //checks if there was a number carried over from the previous iteration
        if carry != 0 {
            //if so, add that to the sum too
            subsum += carry;
        }
        //we only want to write down the last digit of the sum
        let digit_to_append : i32 = subsum.to_string().chars().last().unwrap().to_digit(10).unwrap() as i32;
        //the carry is going to have every digit of the sum *but* the last digit
        let overwrite_carry_with : u128 = subsum.to_string().chars().take(subsum.to_string().chars().count() -1).collect::<String>().parse::<u128>().unwrap();
        carry = overwrite_carry_with;
        //we have the digit to write down, insert it into the answer
        answer.insert(0,digit_to_append);
        //finally, we set the temporary sum back to 0 for the next iteration
        subsum = 0; 
    }
    //is there anything left in the carry? prepend it to the anser
    for x in carry.to_string().chars().rev() {
        answer.insert(0,x.to_digit(10).unwrap() as i32);       
    }
    
    println!("result: {:?}",answer);
    }

///Longest Collatz sequence
/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?
NOTE: Once the chain starts the terms are allowed to go above one million.
*/
#[allow(dead_code)]
    pub fn problem_014() {
        fn nr_of_steps(starting_number: i64) -> i64 {
        let mut starting_number = starting_number; 
        let mut steps = 0;
        loop {
            if starting_number == 1 {
                break;
            }
        match starting_number % 2 {
            0 => {
                starting_number /= 2;
                steps += 1;
            }
            _ => {
                starting_number = (starting_number * 3) + 1;
                steps += 1;
            }
        }
    
        }
        steps
        }
        let mut max = 0;
        let mut val = 0;
        for x in 1..1000000 {
            let steps = nr_of_steps(x);
             if steps > max {
            max = steps;
            val = x;
        }
        }
        println!("{} took {}steps", val,max);
    }
///Lattice paths
#[allow(dead_code)]
    pub fn problem_015() {
        //apparently this sequence looks like "binomial cofficient", (6,20,70,252,924 etc)
        //try to get a function that generates these and check what you should return
        fn iterative() {
        static WIDTH:u32 =20;
        static HEIGHT:u32 = WIDTH;//it's a grid, it's supposed to be square
        static NR_OF_BITS_FOR_ALL_MOVES:u32 = WIDTH + HEIGHT;

        let mut goodmoves =0; 
        for x in 0..(2u64.pow(NR_OF_BITS_FOR_ALL_MOVES)){
            goodmoves += (x.count_ones() == WIDTH) as u32;
         }
        println!("valid moves : {}",goodmoves); 
        
        }
        //find another datatype large enough to fit fac(2*20);
        fn fac(n:u128) -> u128 {
            match n < 2 {
                true => {  1  },
                false => { n*fac(n-1)  }
            }
        }
        fn optimized_solution(n: u128) -> u128{
             fac(2*n) / (fac(n)).pow(2)
        } 
        //iterative();
        println!("{}",optimized_solution(20));
    }
///Power digit sum
#[allow(dead_code)]
    pub fn problem_016() {
         use num_bigint::BigUint; 
        println!("{}",BigUint::new(vec![2]).pow(1000).to_string().chars().map(|x| x.to_digit(10)).fold(0,|sum,value| sum + value.unwrap()));
    }
///Number letter counts
#[allow(dead_code)]
    pub fn problem_017(){
        fn num_to_word(num:i32){
            let words = vec!["one","two","three","four","five","six","seven","eight","nine"];
            let tens = vec!["ten","twenty","thirty","fourty","fifty"];
            let power_of_ten = vec!["hundred","thousand"];
           for (x,y) in num.to_string().chars().enumerate() {
                let pos = num.to_string().len() - x;
                match pos {
                    4 => {
                        print!("{} thousand, ",words[num as usize/ 1000 as usize -1]);
                    },
                    3 => {
                        print!("{} hundred",words[(num - (num / 1000) * 1000) as usize / 100 as usize -1]);
                    }
                    2 => {
                        print!("");
                    }
                    1 => {
                        print!("\n");
                    }
                    _ => {
                    }
                }
            }
        }
        num_to_word(2300);
    }

