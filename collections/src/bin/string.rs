fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let mut s = "initial contents".to_string();
    s.push_str(" bar");

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2[..]; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "박시윤 바보".chars() {
        println!("{}", c);
    }

    let siyoon = "박시윤 바보";
    let s = &siyoon[..9];
    println!("{}", s);
    println!("{}", siyoon.len());
}
