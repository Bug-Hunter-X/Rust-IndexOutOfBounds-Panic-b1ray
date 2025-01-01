fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Check index before accessing the element
    let index = 10;
    if index < vec.len() {
        println!("{}", vec[index]);
    } else {
        println!("Index out of bounds");
    }
    
    //Using get to avoid panic
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index out of bounds")
    }
} 