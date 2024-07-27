fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Ricid Kumbara";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Ricid Kumbara";
    println!("Hello {}", name);

    name = "Fulan";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Ricid Kumbara";
    println!("Hello {}", name);
    
    let name = 10;
    println!("Hello {}", name);
}