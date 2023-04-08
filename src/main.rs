use linear_hashtable_example::*;
fn main() {
    let mut map = HashMap::new();
    map.insert("Apple", 0);
    map.insert("Banana", 1);
    map.insert("Chocolate", 2);
    println!("{:#?}", map);

    map.remove("Banana");
    println!("{:#?}", map);

    map.insert("Dingo", 3);
    println!("{:#?}", map);
}
