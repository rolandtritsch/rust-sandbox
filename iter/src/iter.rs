/// This function implements a simple for loop.
fn for_loop() {
    for _x in 0..10 {
        print!(".");
    }
    println!("");
}

/// This function uses an iterator to go over the range.
fn iter_loop() {
    let mut range = 0..10;

    loop {
        match range.next() {
            Some(x) => {
                print!("{}", x);
            },
            None => {
                println!("");
                break;
            }
        }
    }
}

/// This function uses an iterator to go over a vector.
fn iter_loop2() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for n in &nums {
        print!("{}", *n);
    }

    println!("");
}

/// This function uses a consumer to build the vector and then iterates over it.
fn iter_loop3() {
    let zero_to_nine = (0..10).collect::<Vec<_>>();
    for n in &zero_to_nine {
        print!("{}", *n);
    }
    println!("");

    let one_greater_than_five = zero_to_nine.iter().find(|&x| *x > 5);
    for n in one_greater_than_five {
        print!("{}", n);
    }
    println!("");

    match one_greater_than_five {
        Some(_) => println!("Found one number!"),
        None => println!("Did not find a number!"),
    }

    let all_greater_than_five = zero_to_nine.iter().filter(|&x| *x > 5);
    for n in all_greater_than_five {
        print!("{}", n);
    }
    println!("");
}

/// This function is summing up numbers by folding a range/vector of numbers.
fn iter_fold() {
    let zero_to_nine = (0..10).collect::<Vec<_>>();
    let sum = zero_to_nine.iter().fold(0, |s, x| s + x);
    println!("{}", sum);
}

/// Main function.
fn main() {
    for_loop();
    iter_loop();
    iter_loop2();
    iter_loop3();
    iter_fold();
}
