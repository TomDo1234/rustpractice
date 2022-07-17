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
    //println!("{}",q10(2000000));
    let q11vec : [[usize; 20] ; 20] = [[08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
                            [49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
                            [81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
                            [52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
                            [22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
                            [24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
                            [32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
                            [67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
                            [24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
                            [21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
                            [78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
                            [16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
                            [86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
                            [19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
                            [04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
                            [88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
                            [04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
                            [20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
                            [20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
                            [01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48]];
    println!("{}",q11(q11vec));

}
