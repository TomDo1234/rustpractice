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
                factors.push(num/i);
            }
        }
    }
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
            //println!("{:?}",numdict.get(&i).unwrap().to_string());
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

fn q18(tree : Vec<Vec<i32>>) -> i32 { 
    fn recursion(tree : &Vec<Vec<i32>>,branchindex : usize , level : usize) -> Vec<i32> {
        let mut results = vec![];
        for i in branchindex..branchindex+2 {
            if level == tree.len() - 1 {
                results.push(tree[level][i])
            }
            else if level < tree.len() - 1  {
                let next = recursion(tree,i,level + 1);
                let mut result = tree[level][i];
                if next[0] > next[1] {
                    result += next[0];
                }
                else {
                    result += next[1];
                }               
                results.push(result);
            }
            else {
                println!("sss");
                return vec![0];
            }
        }
        return results;
    }   
    return *recursion(&tree,0,1).iter().max().unwrap() + &tree[0][0];
}

fn q19() -> i32 {
    let normalyearfirstdays = vec![1,32,60,91,121,152,182,213,243,274,304,335];
    let leapyearfirstdays = vec![1,32,61,92,122,153,183,214,244,275,305,336];
    let mut yearcounter = 1900;
    let mut daycounter = 1;
    let mut sundaycounter = 0;
    for i in 0..36500 {
        let yearlength = if yearcounter % 4 == 0 { 366 } else { 365 };
        let yearfirstdays = if yearcounter % 4 == 0 { &leapyearfirstdays } else { &normalyearfirstdays };
        if i % 7 == 0 && yearfirstdays.contains(&daycounter) {
            sundaycounter += 1;
        }
        if daycounter >= yearlength {
            daycounter = 0;
            yearcounter += 1;
        }
        daycounter += 1;
    }
    return sundaycounter;
}

fn q20(upper : i32) -> u32 {
    let mut init = BigInt::from(1 as i32);
    for i in 2..upper + 1 {
        init = init.checked_mul(&BigInt::from(i)).unwrap();
    }
    let string = init.to_str_radix(10);
    return string.chars().map(|ch| ch.to_digit(10).unwrap()).sum();
}



fn q21() -> i32 {
    let mut d_of_ns : Vec<i32> = vec![];
    let mut pairs : Vec<Vec<i32>> = vec![];
    let mut sum = 0;
    for i in 1..10001 {
        let properdivisorsum = divisors(i).iter().sum::<i32>() - i; //get sum of PROPER divisors
        if d_of_ns.contains(&i) {
            for j in 0..i-1 {
                if d_of_ns[j as usize] == i && j + 1 == properdivisorsum {
                    sum += i;
                    sum += j + 1;
                    pairs.push(vec![j + 1,i]);
                }
            }
        }
        d_of_ns.push(properdivisorsum); 
    }
    println!("{:?}",pairs);
    return sum;
}

use std::fs;
fn q22() -> i32 {
    // --snip--
    let filename = "./src/p022_names.txt";

    let lettervalue : HashMap<char,i32> = HashMap::from([('A',1),('B',2),('C',3),('D',4),('E',5),('F',6),('G',7),
    ('H',8),('I',9),('J',10),('K',11),('L',12),('M',13),('N',14),
    ('O',15),('P',16),('Q',17),('R',18),('S',19),('T',20),('U',21),('V',22),('W',23),('X',24),('Y',25),('Z',26)]);

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content.replace('"',""),
        Err(error) => panic!("Problem reading the file: {:?}", error)
    };

    let mut contents : Vec<&str> = contents.split(",").collect();
    contents.sort();


    let mut values : Vec<i32> = vec![];
    for i in 0..contents.len() {
        let wordvalue : i32 = contents[i].chars().map(|x| lettervalue.get(&x).unwrap()).sum();
        values.push((i as i32 + 1) * wordvalue);
    }
    return values.iter().sum();
}

use std::collections::HashSet;

fn q23() -> i32 {
    let mut thelist : Vec<i32> = vec![];
    let mut thenumbers = HashSet::new();
    for i in 1..28124 { //14063 is 1 more than half of 28123
        if divisors(i).iter().sum::<i32>() - i > i {
            thelist.push(i);
        }
    }
    for i in 0..thelist.len() {
        for j in i..thelist.len() {
            let num = thelist[i]+thelist[j];
            if num <= 28123 {
                thenumbers.insert(num);
            }
        }
    }

    return (1..28124).sum::<i32>() - thenumbers.iter().sum::<i32>(); 
}

fn factorial(num : usize) -> usize {
    let mut res = 1;
    for i in 1..num + 1 {
        res *= i;
    }   
    return res;
}

fn q24() -> String {
    let mut digits : Vec<i32> = vec![];
    let mut remainingdigits = vec![0,1,2,3,4,5,6,7,8,9];
    let mut i = 0;
    let mut facbase = remainingdigits.len();
    while i < 1000000 && facbase > 0 {
        let mut mult = 0;
        let fac = factorial(facbase);
        let mut j = fac;
        while i + j < 1000000 {
            mult += 1;
            j = mult * fac;
        }
        if mult != 0 {
            i += (mult - 1) * fac; 
            digits.push(remainingdigits[(mult - 1) as usize]);
            remainingdigits.remove((mult - 1) as usize);
        }
        facbase -= 1;
    }
    digits.push(remainingdigits[0]);
    return digits.iter().map(|v| v.to_string()).collect::<String>(); //covert vector of digits to number
}

fn main() {
    //println!("{}",q19());
    //println!("{:?}",q21());
    // println!("{:?}",q22());
    //println!("{}",q23()); 2783915460
    println!("{}",q24());
}
