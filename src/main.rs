use linear_hashtable_example::*;
fn main() {
    let apple = "Apple".to_string();
    let banana = "Banana".to_string();
    let chocolate = "Chocolate".to_string();
    let dingo = "Dingo".to_string();
    let egg: String = "Egg".to_string();
    let fred: String = "Fred".to_string();
    let greg: String = "Greg".to_string();
    let hummus: String = "Hummus".to_string();
    let indigo: String = "Indigo".to_string();
    let jewel: String = "Jewel".to_string();

    let mut map = HashMap::new();
    println!("{:?}", map);

    map.insert(apple, 0);
    println!("{:?}", map);

    map.insert(banana, 1);
    println!("{:?}", map);

    map.insert(chocolate, 2);
    println!("{:?}", map);

    println!("Apple: {:?}", map.get("Apple"));
    println!("Banana: {:?}", map.get("Banana"));
    println!("Chocolate: {:?}", map.get("Chocolate"));

    map.remove("Banana");
    println!("{:?}", map);

    println!("Apple: {:?}", map.get("Apple"));
    println!("Banana: {:?}", map.get("Banana"));
    println!("Chocolate: {:?}", map.get("Chocolate"));

    map.insert(dingo, 3);
    println!("{:?}", map);

    println!("Apple: {:?}", map.get("Apple"));
    println!("Banana: {:?}", map.get("Banana"));
    println!("Chocolate: {:?}", map.get("Chocolate"));
    println!("Dingo: {:?}", map.get("Dingo"));

    map.insert(egg, 4);
    map.insert(fred, 5);
    map.insert(greg, 6);
    map.insert(hummus, 7);
    map.insert(indigo, 8);
    println!("{:?}", map);

    map.insert(jewel, 9);
    println!("{:?}", map);

    map.remove("Apple");
    map.remove("Banana");
    map.remove("Chocolate");
    map.remove("Dingo");
    map.remove("Fred");
    map.remove("Greg");
    println!("{:?}", map);

    map.remove("Jewel");
    println!("{:?}", map);

    map.remove("Indigo");
    map.remove("Hummus");
    println!("{:?}", map);

    map.remove("Egg");
    println!("{:?}", map);
}
