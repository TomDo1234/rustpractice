#![allow(dead_code)] //I want to focus on most recent question, so some funcs are "unused"


fn q1(ceilnum : i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..ceilnum {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    return sum;
}

fn q2(ceilnum : i32) -> i32 {
    let mut sumbefore : i32 = 1;
    let mut sum: i32 = 1;
    let mut evensum : i32 = 0;
    while sum <= ceilnum {
        let oldsumbefore = sumbefore;
        sumbefore = sum;
        sum += oldsumbefore;
        if sum % 2 == 0 {
            evensum += sum;
        }
    }
    return evensum;
}

fn nthprime(n : i64) -> i64 {
    let mut prevprimes = vec![2];
    let mut iter : i64 = 1;
    let mut num : i64 = 2;
    while iter < n {
        num += 1;
        if prevprimes.iter().all(|x| num % x != 0) {
            prevprimes.push(num);
            iter += 1;
        }
    }
    return num;
}

fn q3(mut primenum : i64) -> i64 {
    let mut n : i64 = 1;
    let mut divisor : i64 = nthprime(n);
    while primenum >= divisor {
        n += 1;
        divisor = nthprime(n);
        if primenum % divisor == 0 {
            primenum /= divisor;
        }
    }
    return divisor;
}

fn q4() -> i32 {
    let mut num = 0;
    let mut biggestnum = num;
    for x in 100..1000 {
        for y in 100..1000 {
            num = x * y;
            let strnum : String = format!("{:?}", num);
            let reversed = strnum.chars().rev().collect::<String>();
            if strnum == reversed && num > biggestnum {
                biggestnum = num;
            }
        }
    }
    return biggestnum
}

fn q5(ceilnum : i32) -> i32 {
    let nums : Vec<i32> = (1..ceilnum + 1).collect();
    let mut finalnum = ceilnum;
    while nums.iter().any(|x| finalnum % x != 0)  {
        finalnum += ceilnum;
        //println!("{}",finalnum);
    }
    return finalnum;
}

fn q6(num : i32) -> i32 {
    let nums : Vec<i32> = (1..num + 1).collect();
    let summedsquares : i32 = nums.iter().map(|x| i32::pow(*x,2)).collect::<Vec<i32>>().iter().sum();
    return i32::pow(nums.iter().sum() , 2) - summedsquares;
}

fn q7(n : i64) -> i64 {
    return nthprime(n);
}

fn q8(strnum : &str) -> u64 {
    let mut maxnum : u64 = 0;
    let string : Vec<u64> = strnum.chars().filter(|c| !c.is_whitespace()).flat_map(|c| c.to_digit(10).map(u64::from)).collect();
    for index in 0..string.len() - 12 {
        let product = string[index..index+13].iter().product();
        if product > maxnum {
            maxnum = product
        }
    }
    return maxnum;
}

fn q9(sum : i32) -> i32 {
    let failure : i32 = -1;
    for a in 1..sum {
        for b in 1..sum {
            let c : f32 = ((a.pow(2) + b.pow(2)) as f32).sqrt();
            if c.fract() == 0.0 { //check no fractional parts, meaning it is an int
                if a + b + c as i32 == 1000 {
                    return a * b * c as i32;
                }
            }
        }
    } 
    return failure;
}



fn primelist(n : usize) -> Vec<usize> {
    let mut boolvec = vec![true ; n];
    boolvec [1] = false; //1 is not prime!
    let bound = (n as f64).sqrt().ceil();
    for i in 2..(bound as usize) {
        if boolvec[i] {
            let mut j = i.pow(2);
            while j < n {
                boolvec[j] = false;
                j += i;
            }
        }
    }
    return boolvec.iter().enumerate().filter(|(_ , &r)| r == true).map(|(index , _ )| index).collect();
}

fn q10(n : usize) -> usize {
    return primelist(n).iter().sum();
}

fn main() {
    //println!("{}",q1(1000));
    //println!("{}",q2(4000000));
    //println!("{}",q3(600851475143));
    //println!("{}",q4());
    //println!("{}",q5(20));
    //println!("{}",q6(100));
    //println!("{}",q7(10001));
    // println!("{}",q8("73167176531330624919225119674426574742355349194934
    //                 96983520312774506326239578318016984801869478851843
    //                 85861560789112949495459501737958331952853208805511
    //                 12540698747158523863050715693290963295227443043557
    //                 66896648950445244523161731856403098711121722383113
    //                 62229893423380308135336276614282806444486645238749
    //                 30358907296290491560440772390713810515859307960866
    //                 70172427121883998797908792274921901699720888093776
    //                 65727333001053367881220235421809751254540594752243
    //                 52584907711670556013604839586446706324415722155397
    //                 53697817977846174064955149290862569321978468622482
    //                 83972241375657056057490261407972968652414535100474
    //                 82166370484403199890008895243450658541227588666881
    //                 16427171479924442928230863465674813919123162824586
    //                 17866458359124566529476545682848912883142607690042
    //                 24219022671055626321111109370544217506941658960408
    //                 07198403850962455444362981230987879927244284909188
    //                 84580156166097919133875499200524063689912560717606
    //                 05886116467109405077541002256983155200055935729725
    //                 71636269561882670428252483600823257530420752963450"));
    //println!("{:?}",q9(1000));
    println!("{}",q10(2000000));
}
