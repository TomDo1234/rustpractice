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

fn main() {
    //println!("{}",q1(1000));
    //println!("{}",q2(4000000));
    //println!("{}",q3(600851475143));
    println!("{}",q4());
}
