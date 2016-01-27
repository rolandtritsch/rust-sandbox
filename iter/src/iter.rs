/// This function implements a simple for loop.
fn for_loop() {
    let range = 0..10;
    for _x in range {
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
fn iter_filter() {
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

/// This function maps an iterator (a range) to a new iterator and doubles the numbers
fn iter_map() {
    let double = (1..5).map(|x| x * 2);
    for n in double {
        print!("{}", n);
    }
    println!("");
}

/// Main function. Calling all the other functions.
fn main() {
    for_loop();
    iter_loop();
    iter_loop2();
    iter_filter();
    iter_fold();
    iter_map();
}
