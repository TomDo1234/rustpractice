
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

fn main() {
    println!("{}",q1(1000));
    println!("{}",q2(1000));
}
