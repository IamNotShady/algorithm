//use rand::Rng;
//use std::cmp::Ordering;
//use std::io;

use std::collections::HashMap;

fn main() {
    //    println!("Guess the number!");
    //    let secret_number = rand::thread_rng().gen_range(1, 101);
    //    println!("The secret number is: {}", secret_number);
    //    loop {
    //        println!("Please input your guess.");
    //        let mut guess = String::new();
    //        io::stdin()
    //            .read_line(&mut guess)
    //            .expect("Failed to read line");
    //        let guess: u32 = match guess.trim().parse() {
    //            Ok(num) => num,
    //            Err(_) => continue,
    //        };
    //        println!("You guessed: {}", guess);
    //        match guess.cmp(&secret_number) {
    //            Ordering::Less => println!("Too small!"),
    //            Ordering::Greater => println!("Too big!"),
    //            Ordering::Equal => {
    //                println!("You win!");
    //                break;
    //            }
    //        }
    //    }
    // 绝对路径
    crate::sound::instrument::clarinet();

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let data = "initial contents";
    let s = data.to_string();
    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
