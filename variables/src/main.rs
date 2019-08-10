fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x += 1;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for i in a.iter() {
        println!("The value of i is: {}", i);
    }

    let a = [3; 5];
    for i in a.iter() {
        println!("The value of i is: {}", i);
    }

    let b = another_function();
    println!("The value of b is: {}", b);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let r#if = 3;
    println!("The value of if is: {}", r#if);

    if 3 > 4 {
        println!("The value of if is: {}", r#if);
    } else if 4 < 5 {
        println!("The value of if is: {}", r#if);
    } else {
        println!("The value of if is: {}", r#if);
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


    let n = 5;
    let f = fibonazzi(n);
    println!("The n th fibonazzi is {}", f);
}

fn another_function() -> i32 {
    println!("Another function.");
    3
}

fn fibonazzi(n: u64) -> u64 {
    let mut current_n = 0;
    let mut previous = 0;
    let mut current = 1;
    while current_n < n {
        current_n += 1;

        let temp = previous;
        previous = current;
        current += temp;
    }
    return current;
}
