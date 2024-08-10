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

#[test]
fn constant() {
    const PI: f32 = 3.14;
    println!("{}", PI)
}

#[allow(dead_code)]
fn function_a() {
    let a = 10;
    let b = String::from("Ricid");

    println!("{} {}", a, b);
}

#[allow(dead_code)]
fn function_b() {
    let a = 10;
    let b = String::from("Kumbara");
    println!("{} {}", a, b);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

#[test]
fn string_test() {
    // string slice
    let name: &str = "  Ricid Kumbara  ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "Ricid";
    println!("{}", username);
    username = "Kumbara";
    println!("{}", username);
}

#[test]
fn string_type() {
    // &str disimpoan di stack
    // String:: disimpan di heap

    let mut name: String = String::from("Ricid Kumbara");
    name.push_str(" Kagenou");
    println!("{}", name);
    
    let new_name: String = name.replace("Ricid", "Ricid_");
    println!("{}", new_name);
}

#[test]
fn ownership_rules() {
    // println!("{}", a);
    let a: i32 = 1;
    
    {
        let b: i32 = 2;
        println!("{}", b); 
    }
    
    // println!("{}", b); 
    println!("{}", a);
}

#[test]
fn ownership_movement() {
    let name: String = String::from("Ricid");
    println!("{}", name);

    // ownership dari name dipindahkan ke name2
    let name2: String = name;
    
    // name1 tidak bisa diakses disini
    // println!("{}", name);
    println!("{}", name2);
}

#[test]
fn clone() {
    // untuk melakukan copy data yang ada di HEAP
    let name1 = String::from("Ricid");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 8 {
        println!("Good");
    } else {
        println!("Not Good");
    }

    
    // Let statement
    let result: &str = if value > 8 {
        "Good"        
    } else if value > 5 {
        "Not Good"
    } else {
        "Very Good"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;
            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}