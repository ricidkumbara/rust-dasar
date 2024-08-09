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

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);
    
    let b: i16 = a as i16;
    println!("{}", a);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a: i8 = 10;
    let b: i8 = 5;
    let c: i8 = a * b;

    println!("{}", (a * b));
    println!("{}", a * b);
    println!("{}", c);

    let mut d: i8 = 2;
    d += 2;
    d *= 2;

    println!("{}", d);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = true;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let result: bool = 10 > 20;

    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final);   
}

#[test]
fn char() {
    let char1: char = 'A';
    let char2: char = 'B';

    println!("{} {}", char1, char2);
}

#[test]
fn tupple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    
    data = (1, 1.5, false);
    println!("{:?}", data);
    println!("{} {} {}", data.0, data.1, data.2);

    data.0 = 2;
    data.1 = 2.5;
    data.2 = true;
    println!("{} {} {}", data.0, data.1, data.2);
}

#[test]
fn tupple_destructuring() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    let (d, _, _) = data;
    println!("{}", d);
}

#[allow(dead_code)]
fn unit() {
    // Tuple kosong biasa disebut unit
    println!("Hello");
}

#[test]
fn test_unit() {
    let result: () = unit();
    println!("{:?}", result);
    
    let test: () = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    println!("{} {} {}", array[0], array[0], array[0]);

    let mut persons = ["Ricid", "Kumbara"];
    persons[1] = "Kumbara_";
    println!("{:?}", persons);
}

#[test]
fn array_length() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let length: usize = array.len();

    println!("{}", length)
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4],
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}