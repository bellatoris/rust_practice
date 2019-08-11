fn main() {
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    if let Some(third) = v.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("There is no third element.");
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let _first = &v[0];

    v.push(6);

    // println!("The first element is: {}", _first);
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    for i in v.into_iter().map(|x| x + 1) {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        print!("{:?} ", i);
    }
}

#[derive(Debug)]
struct SiYoon {
}
