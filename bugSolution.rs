fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; 
        *y = 10;
    }
    { // Create another scope 
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}