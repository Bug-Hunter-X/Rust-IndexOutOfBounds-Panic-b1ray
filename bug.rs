fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //This will cause a panic because we are trying to access index 10
    println!("{}", vec[10]);
}