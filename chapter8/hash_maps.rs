fn main() {
    use std::collections::HashMap;

    let mut s = HashMap::new();

    s.insert(String::from("blue"), 10); //for owned types like strings, the map owns the value
    s.insert(String::from("red"), 20);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10,20];

    let scores: HashMap<_,_> = teams.into_iter().zip(init_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //the get method returns an option enum for the case the key does not exist

    for (key,value) in &scores {
        println!("{}: {}",key,value);
    }

    // there are several options of what to do when inserting a value to a key that
    // already exists in the HashMap: Here are some examples

    // OVERWRITING THE VALUE
    let mut s2 = HashMap::new();
    s2.insert(String::from("1"), 1);
    s2.insert(String::from("1"), 2);

    println!("{:?}", s2);

    // ONLY INSERT IF THE KEY HAS NO VALUE 
    let mut s3 = HashMap::new();
    s3.insert(String::from("1"), 1);

    s3.entry(String::from("2")).or_insert(2);
    s3.entry(String::from("1")).or_insert(3);

    println!("{:?}",s3);

    // UPDATING THE VALUE BASED ON THE PREVIEWS VALUE   
    let text = "hello world wonderful world";

    let mut s4 = HashMap::new();

    for word in text.split_whitespace() {
        let count = s4.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",s4);
}