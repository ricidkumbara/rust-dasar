mod first;
mod second;
mod third;
mod model;

use first::say_hello as say_hello_first;
use second::say_hello as say_hello_second;


fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn variable() {
    let name = "Ricid Kumbara";
    println!("Hello {}", name);
}

#[test]
fn variable_mutable() {
    let mut name = "Ricid Kumbara";
    println!("Hello {}", name);

    name = "Fulan";
    println!("Hello {}", name);
}

#[test]
fn variable_shadowing() {
    let name = "Ricid Kumbara";
    println!("Hello {}", name);
    
    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn variable_implicit_type_declaration() {
    let a = 1;

    println!("{}", a);
}

#[test]
fn variable_explicit() {
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
    println!("{}", d);
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

#[test]
fn while_loop() {
    let mut counter = 1;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }

    for value in array {
        println!("{}", value);
    }
}

#[test]
fn range_exclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
 
    // Range Exclusive
    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Value : {} | Index: {}", array[i], i);
    }
}

#[test]
fn range_inclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
 
    // Range Inclusive
    let range = 0..=4;

    for i in range {
        println!("Value : {} | Index: {}", array[i], i);
    }
}

#[allow(dead_code)]
fn say_hello() {
    println!("Hello");    
}

#[test]
fn test_say_hello() {
    say_hello();
}

#[allow(dead_code)]
fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test() {
    say_goodbye("Ricid", "Kumbara");
}

#[allow(dead_code)]
fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return  0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);

    let result: i32 = factorial_loop(-5);
    println!("{}", result);
}

#[allow(dead_code)]
fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Ricid"), 10);
}

#[allow(dead_code)]
fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursivw() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

#[allow(dead_code)]
fn print_number(number: i32) {
    println!("{}", number);
}

#[allow(dead_code)]
fn hi(name: String) {
    println!("{}", name);
}

#[test]
fn test_hi() {
    let number: i32 = 10;
    print_number(number);
    println!("{}", number);

    let name: String = String::from("Ricid");
    hi(name);
    // println!("{}", name); owner sudah dimove, tidak bisa dipanggil
}

#[allow(dead_code)]
fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Ricid");
    let last_name = String::from("Kumbara");
    let name = full_name(first_name, last_name);

    println!("{}", name);
    // println!("{}", first_name); // owner sudah dimove, tidak bisa dipanggil
    // println!("{}", last_name); // owner sudah dimove, tidak bisa dipanggil
}

#[allow(dead_code)]
fn full_name_2(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name_2() {
    // Cara agar ownership bisa dikembalikan
    let first_name = String::from("Ricid");
    let last_name = String::from("Kumbara");
    let (first_name, last_name, full_name) = full_name_2(first_name, last_name);

    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}

#[allow(dead_code)]
fn full_name_using_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name_using_reference() {
    let first_name: String = String::from("Ricid");
    let last_name: String = String::from("Kumbara");
    let name: String = full_name_using_reference(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[allow(dead_code)]
fn change_value(value: &mut String) {
    value.push_str("Test")    
}

#[test]
fn test_change_value() {
    let mut value: String = String::from("Ricid");
    println!("{}", value);
 
    change_value(&mut value);
    println!("{}", value);
 
    value.push_str("_");
    println!("{}", value);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let slice1: &[i32] = &array[..];
    let slice2: &[i32] = &array[0..5];
    let slice3: &[i32] = &array[5..];

    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Ricid Kumbara");
    let first_name: &str = &name[0..3];
    let last_name: &str = &name[10..];

    println!("{}", first_name);
    println!("{}", last_name);
}

#[allow(dead_code)]
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

#[allow(dead_code)]
impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

#[test]
fn struct_person_common() {
    let person: Person = Person { 
        first_name: String::from("Ricid"), 
        middle_name: String::from("Kumbara"), 
        last_name: String::from("Kagenou"), 
        age: 26 
    };
    
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.middle_name);
    println!("{}", person.age);

    let first_name = String::from("Ricid");
    let last_name: String = String::from("Kumbara");
    let mut person2: Person = Person { 
        first_name, 
        middle_name: String::from("Kagenou"), 
        last_name, 
        age: 26
    };

    person2.first_name = String::from("Ricid_");
    println!("{}", person2.first_name);

    let person3 = Person{..person};
    println!("{}", person3.first_name);
}

#[allow(dead_code)]
fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.middle_name);
    println!("{}", person.age);
}

#[test]
fn struct_person_function() {
    let person: Person = Person { 
        first_name: String::from("Ricid"), 
        middle_name: String::from("Kumbara"), 
        last_name: String::from("Kagenou"), 
        age: 26 
    };
    
    print_person(&person);
}

// Struct Tupple
#[allow(dead_code)]
struct GeoPoint(f64, f64);

#[allow(dead_code)]
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn struct_tupple() {
    let geo_point: GeoPoint = GeoPoint(-6.0001, 100.123434);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[allow(dead_code)]
struct Nothing;

#[test]
fn struct_without_field() {
    let _nothing: Nothing = Nothing;
    let _nothing: Nothing = Nothing {};
}

#[test]
fn struct_with_method() {
    let person: Person = Person { 
        first_name: String::from("Ricid"), 
        middle_name: String::from("Kumbara"), 
        last_name: String::from("Kagenou"), 
        age: 26 
    };

    person.say_hello("Fulan");
    println!("{}", person.first_name);
}

#[test]
fn struct_assosiated_function() {
    let geo_point = GeoPoint::new(6.1001, 6.1001);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[allow(dead_code)]
enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn enum_test() {
    let level: Level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

#[allow(dead_code)]
enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

#[allow(dead_code)]
impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amound {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amound {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with credit card {} {} amound {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn enum_with_value() {
    let _payment1: Payment = Payment::CreditCard(String::from("123"));
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("123"));

    _payment1.pay(1000);
    _payment2.pay(2000);
}

#[test]
fn match_value() {
    let name = "Kagenou";

    match name {
        "Ricid" | "Kagenou" => {
            println!("Hello Pipe!!!");
        }
        "Kumbara" => {
            println!("Hello Kumbara");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn match_range_pattern() {
    let value = 100;

    match value {
        75..=100 => {
            println!("Great");    
        }
        50..=74 => {
            println!("Good");
        }
        0..=49 => {
            println!("Bad");
        }
        other => {
            println!("Invalid Value {}", other);
        }
    }
}

#[test]
fn match_struct_destructuring() {
    let point: GeoPoint = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("Long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("Lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("Long : {} Lat: {}", long, lat);
        }
    }

    let person = Person{
        first_name: String::from("Ricid"),
        middle_name: String::from("Kumbara"),
        last_name: String::from("Kagenou"),
        age: 20,
    };

    match person {
        Person { first_name, last_name, .. } => {
            println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn match_struct_destructuring_ignoring() {
    let point: GeoPoint = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            println!("Long: {}", long);
        }
    }
}

#[test]
fn match_struct_destructuring_ignoring_range() {
    let value = 101;

    match value {
        75..=100 => {
            println!("Great");    
        }
        50..=74 => {
            println!("Good");
        }
        0..=49 => {
            println!("Bad");
        }
        _ => {
            println!("Invalid Value",);
        }
    }
}

#[test]
fn match_exression() {
    let value = 2;

    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };
    println!("{}", result);
}

#[allow(dead_code)]
type Age = u8;
#[allow(dead_code)]
type IdentityNumber = String;

#[allow(dead_code)]
struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn type_alias() {
    let customer: Customer = Customer { 
        id: String::from("32001"), 
        name: String::from("Ricid Kumbara"), 
        age: 25,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

#[test]
fn module_test() {
    let user: model::User = model::User {
        first_name: String::from("Ricid"),
        username: String::from("ricid"),
        email: String::from("ricidkumbara@gmail.com"),
        age: 20,
    };

    user.say_hello("Fulan");
}

#[test]
fn module_use() {
    first::say_hello();
    second::say_hello();

    say_hello_first();
    say_hello_second();
}