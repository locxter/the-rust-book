use std::collections::HashMap;

fn main() {
    // Hash maps can be understood as a supercharged version of vectors, which uses keys of any type to identify a value
    // instead of always using an index
    let mut map = HashMap::new();
    // Values are added with .insert()
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    println!("{:?}", map);
    // And can be accessed with .get(), which returns an Option<&V> (or an Option<T> if paired with .copied())
    let team = String::from("Blue");
    let score = map.get(&team).copied();
    match score {
        Some(score) => println!("{team}: {score}"),
        None => println!("{team}: Score not found"),
    }
    // Iterating is very similar to vectors:
    for (key, value) in &map {
        println!("{key}: {value}");
    }
    // As the data is stored in heap, types that do not implement the Copy trait will be moved into the hash map and
    // therefore owned by it
    let key = String::from("Key");
    let value = String::from("Value");
    let mut other_map = HashMap::new();
    other_map.insert(key, value);
    // This will cause an error since the data is owned by the map now:
    // println!("{key}: {value}");
    // By default, new values overwrite old ones
    map.insert(String::from("Blue"), 25);
    println!("{:?}", map);
    // You can also choose to only add a value if its key does not already exist by using .or_insert()
    map.entry(String::from("Yellow")).or_insert(50);
    map.entry(String::from("Red")).or_insert(100);
    println!("{:?}", map);
    // Alternatively, it is also possible to update value using a mutuable reference
    let text = "hello world wonderful world";
    let mut other_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = other_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", other_map);
}
