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

fn q11(grid : [[usize; 20] ; 20]) -> usize {
    let mut maxnum = 0;
    for i in 0..20 {
        for j in 0..17 {
            let horproduct = grid[i][j..j+4].iter().product::<usize>();
            if horproduct > maxnum {
                maxnum = horproduct;
            }
        }
    }
    for i in 0..17 {
        for j in 0..20 {
            let verproduct = [grid[i][j],grid[i+1][j],grid[i+2][j],grid[i+3][j]].iter().product::<usize>();
            if verproduct > maxnum {
                maxnum = verproduct;
            }
        }
    }
    for i in 0..17 {
        for j in 0..17 {
            let diaproduct = [grid[i][j],grid[i+1][j+1],grid[i+2][j+2],grid[i+3][j+3]].iter().product::<usize>();
            if diaproduct > maxnum {
                maxnum = diaproduct;
            }
        }
    }

    for i in 0..17 {
        for j in 3..20 {
            let diaproduct = [grid[i][j],grid[i+1][j-1],grid[i+2][j-2],grid[i+3][j-3]].iter().product::<usize>();
            if diaproduct > maxnum {
                maxnum = diaproduct;
            }
        }
    }
    return maxnum;
}

fn divisors(num : i32) -> Vec<i32> {
    let mut factors = vec![];
    let ceil = (num as f32).sqrt().floor() as i32; //auto floors
    for i in 1..ceil + 1 {
        if num % i == 0 {
            factors.push(i);
            if i != num/i {
                factors.push(i);
            }
        }
    }
    factors.push(num);
    return factors;
}

fn q12(divs : usize) -> i32 {
    let mut num = 0;
    let mut trianglenum = 0;
    while divisors(trianglenum).len() <= divs {
        num += 1;
        trianglenum += num;
    }
    return trianglenum;
}

use num_bigint::{BigInt};

fn q13(list : [BigInt ; 100]) -> String {
    let mut b = &list[0] + &list[1];
    for i in 2..100 {
        b += &list[i];
    }
    return b.to_str_radix(10)[..10].to_string();
}

fn q14(ceilnum : i64) -> i64 {
    let mut maxchainlength = vec![0,1];
    let mut length = 0;
    for i in 1..ceilnum {
        let mut j = i;
        while j != 1 {
            if j % 2 == 1 {
                j = (3*j + 1)/2;
                length += 2;
            }
            else {
                j /= 2;
                length += 1;
            }
        }
        if length > maxchainlength[0] {
            maxchainlength[0] = length;
            maxchainlength[1] = i;
        }
        length = 0;
    }
    return maxchainlength[1];
}

fn q15() -> u128 { //essentially this is 40!/20!/20!
    let mut a = 1;
    let mut b = 1;
    for i in 21..41 { 
        a *= i
    }
    for i in 2..21 {
        b *= i
    }
    return a/b;
}

fn q16() -> u32 {
    let base = BigInt::from(2);
    let result = base.pow(1000);
    return result.to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum();
}

use std::collections::HashMap;
fn q17(num : i32) -> usize {
    let numdict = HashMap::from([
        (0,""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100,"onehundred"),
        (200,"twohundred"),
        (300,"threehundred"),
        (400,"fourhundred"),
        (500,"fivehundred"),
        (600,"sixhundred"),
        (700,"sevenhundred"),
        (800,"eighthundred"),
        (900,"ninehundred"),
        (1000, "onethousand")
    ]);
    let mut words = vec![];
    for i in 1..num + 1 {
        if numdict.contains_key(&i) {
            words.push(numdict.get(&i).unwrap().to_string());
            println!("{:?}",numdict.get(&i).unwrap().to_string());
        }
        else {
            let digits : Vec<u32> = i.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
            let mut word = vec![];
            if digits.len() == 3 { 
                word.push(numdict.get(&(digits[0] as i32)).unwrap().to_string()) ; 
                word.push("hundredand".to_string()) 
            }
            if digits[digits.len() - 2] != 1 {
                word.push(numdict.get(&(digits[digits.len() - 2] as i32 * 10)).unwrap().to_string());
                word.push(numdict.get(&(digits[digits.len() - 1] as i32)).unwrap().to_string());
            }
            else {
                word.push(numdict.get(&((digits[1] * 10 + digits[2]) as i32)).unwrap().to_string());
            }          
            words.push(word.join(""));
            //println!("{:?}",word.join(""));
        }
    }
    let string = words.join("");
    return string.chars().count();
}

fn main() {
    //println!("{}",q14(1000000));
    //println!("{}",q15());
    //println!("{}",q16());
    //println!("{}",q17(1000));
    println!("{}",q17(1000));
   

}
