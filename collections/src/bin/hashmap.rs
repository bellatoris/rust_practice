use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}, {:?}", map, field_name);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &mut scores {
        println!("{}: {}", key, value);
        *value += 30;
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let v = vec![1, 2, 3, 4, 5, 6];
    println!("max value is {}", get_max(&v));
    println!("mean value is {}", get_mean(&v));

    let v = vec![1, 2, 3, 4, 5, 6];
    println!("sorted value is {:?}", quick_sort(&v));

    let v = vec![12, 23, 13, 4, 5, 6];
    println!("median value is {:?}", get_median(&v));

    let v = vec![1, 2, 3, 4, 5, 6];
    println!("median value is {:?}", get_median(&v));

    let v = vec![1, 2, 3, 4, 5];
    println!("median value is {:?}", get_median(&v));

    let v = vec![1, 1, 2, 2, 2];
    println!("most value is {:?}", get_mode(&v));

    let v = vec![1, 1, 2, 2, 2];
    println!("sorted value is {:?}", quick_sort(&v));

    println!("{}", pigify_one(&String::from("박시윤")));
    println!("{}", pigify_one(&String::from("first")));
    println!("{}", pigify_one(&String::from("apple")));
    println!("{}", pigify_one(&String::from("नमस्ते")));
}

fn get_max(vec: &Vec<i32>) -> i32 {
    let mut result = std::i32::MIN;
    for i in vec {
        if result < *i {
            result = *i;
        }
    }
    result
}

fn get_mean(vec: &Vec<i32>) -> f64 {
    let length = vec.len();
    let mut sum = 0;

    for i in vec {
        sum += *i;
    }
    sum as f64 / length as f64
}

// 실제 구현은 vector 내부에 있는 array 에 접근해서 swap 을 통해 이루어진다.
fn quick_sort(vec: &Vec<i32>) -> Vec<i32> {
    fn nested_fun(vec: Vec<i32>) -> Vec<i32> {
        if vec.len() <= 1 {
            return vec;
        }

        let pivot = &vec.get(0).expect("박시윤 바보");
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut pivot_list = Vec::new();

        for i in &vec {
            if i > pivot {
                left.push(*i);
            } else if i < pivot {
                right.push(*i);
            } else {
                pivot_list.push(*i);
            }
        }

        let mut left_sorted = nested_fun(left);
        let mut right_sorted = nested_fun(right);
        
        left_sorted.append(&mut pivot_list);
        left_sorted.append(&mut right_sorted);
        left_sorted
    }

    nested_fun(Vec::from_iter(vec.into_iter().map(|x| *x)))
}

fn get_median(vec: &Vec<i32>) -> f64 {
    let vec = quick_sort(vec);
    let length = vec.len();
    if length % 2 == 0 {
        let result = vec[length / 2 - 1] + vec[length / 2];
        result as f64 / 2 as f64
    } else {
        vec[length / 2] as f64
    }
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in vec {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut result = std::i32::MIN;
    let mut result_key = 0;

    for (key, value) in &map {
        if *value > result {
            result = *value;
            result_key = *key;
        }
    }

    println!("{:?}", map);
    result_key
}

fn pigify_one(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}
