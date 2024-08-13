mod first;
mod second;
mod third;
mod model;

use first::say_hello as say_hello_first;
use second::say_hello as say_hello_second;
use core::ops::Add;
use std::{cell::RefCell, collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque}, fmt::Debug, ops::Deref, rc::Rc};

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

    // Tipe data yg ada di stack
    let x: i32 = 10;
    let y: &i32;

    y = &x;

    println!("{}", x);
    println!("{}", y);
    println!("{}", x);

    // Tipe data yg ada di stack
    let a: String = "Fulan".to_string();
    println!("{}", a);

    let b: &String;
    b = &a;
    println!("{}", a);
    println!("{}", b);
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
    
    // name tidak bisa diakses disini
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
    first::first_second::first_third::say_hello();
    second::say_hello();

    say_hello_first();
    say_hello_second();
}

#[allow(dead_code)]
trait CanSayHello {
    // Default Trait Implementation
    fn hello(&self) -> String {
        String::from("Hello")
    }

    fn say_hello_trait(&self) -> String;
    fn say_hello_trait_to(&self, name: &str) -> String;
}

#[allow(dead_code)]
trait CanSayGoodbye {
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello_trait(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_trait_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodbye for Person {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

#[allow(dead_code)]
fn say_hello_trait_new(value: &impl CanSayHello) {
    println!("{}", value.say_hello_trait());
}

#[allow(dead_code)]
fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello_trait());
    println!("{}", value.good_bye());
}

#[test]
fn trait_test() {
    let person: Person = Person { 
        first_name: String::from("Ricid"), 
        middle_name: String::from("Kagenou"), 
        last_name: String::from("Kumbara"), 
        age: 25 
    };

    println!("{}", person.hello());
    println!("{}", person.say_hello_trait());
    println!("{}", person.say_hello_trait_to("Fulan"));

    // Trait as parameter
    say_hello_trait_new(&person);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Fulan"));

    // Multiple trait parameter
    hello_and_goodbye(&person);

    CanSayHello::say_hello_trait(&person);
    Person::say_hello(&person, "Ricid");
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

#[allow(dead_code)]
fn create_person(name: String) -> impl CanSayGoodbye {
    SimplePerson { name }
}

#[test]
fn trait_return_value() {
    let person = create_person(String::from("Ricid"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Ricid"));
}

// trait CanSay: CanSayHello + CanSayGoodbye {}

// Stuct with default type
#[allow(dead_code)]
struct Point<T = i32> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn generic_struct() {
    let integer: Point<i32> = Point::<i32> {
        x: 5,
        y: 10,
    };

    let float: Point<f64> = Point::<f64> {
        x: 5.1,
        y: 10.2,
    };

    println!("{}", integer.x);
    println!("{}", integer.y);
    println!("{}", float.x);
    println!("{}", float.y);
}

#[allow(dead_code)]
enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn generic_enum() {
    let value:Value<i32> = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("None")
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

#[allow(dead_code)]
struct HI<T> where T: CanSayGoodbye {
    value: T,
}

#[test]
fn generic_bound() {
    let hi = HI::<SimplePerson> {
        value: SimplePerson { 
            name: String::from("Ricid")
        }
    };

    println!("{}", hi.value.name);
}

#[allow(dead_code)]
fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);

    let result = min(10.1, 20.2);
    println!("{}", result);
}

#[test]
fn generic_method() {
    let point = Point{x: 10, y: 11};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

#[allow(dead_code)]
trait GetValue<T> {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;    

    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn overloadable_operators() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 10};

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

#[allow(dead_code)]
fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn option() {
    // let result = double(Some(10));   
    let result = double(Option::Some(10));   
    println!("{:?}", result);

    let result = double(None);   
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }   
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn comparing() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("{}", apple1 == apple2);
    println!("{}", apple1 < apple2);
    println!("{}", apple1 > apple2);
}

#[test]
fn string_manipulation() {
    let s = String::from("Ricid Kumbara");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Ricid", "_"));
    println!("{}", s.contains("Ricid"));
    println!("{}", s.ends_with("Kumbara"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));
}

#[allow(dead_code)]
struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }   
}

#[test]
fn formatting() {
    let category = Category {
        id: String::from("1"),
        name: String::from("HP"),
    };

    println!("{:?}", category);
}

#[test]
fn closure() {
    // Closure juga biasa disebut anonimous function
    let sum = |value1: i32, value2: i32| -> i32 {
        value1 + value2
    };

    let result = sum(1, 2);
    println!("{}", result);
}

#[allow(dead_code)]
fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn closure_as_parameter() {
    print_with_filter(String::from("Ricid"), |value: String| -> String {
        value.to_ascii_uppercase()
    });
}

#[allow(dead_code)]
fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn closure_with_existring_function() {
    print_with_filter(String::from("Ricid"), to_uppercase);
}

#[test]
fn closure_scoope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };
    
    increment();
    increment();
    increment();

    print!("{}", counter);
}

#[allow(dead_code)]
struct Counter {
    counter: i32,
}

#[allow(dead_code)]
impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn counter_test() {
    let mut counter = Counter{counter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("{}", counter.counter);
}

#[test]
fn vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Ricid"));
    names.push(String::from("Kumbara"));
    names.push(String::from("Kagenou"));

    // Cara ini akan memindahkan ownership
    // for name in names {
    //     println!("{}", name);
    // }

    // Error, ownership sudah dipindahkan
    // println!("{:?}", names); 

    for name in &names {
        println!("{}", name);
    }

    // Tidak error karena menggunakan references
    println!("{:?}", names);
}

#[test]
fn vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Ricid"));
    names.push_back(String::from("Kumbara"));
    names.push_front(String::from("Kagenou"));

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();
    names.push_back(String::from("Ricid"));
    names.push_back(String::from("Kumbara"));
    names.push_front(String::from("Kagenou"));

    // Tidak bisa akses linked list by index
    // println!("{}", names[1]);

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn hash_map() {
    // Akses data bisa menggunakan index
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("Ricid"));
    map.insert(String::from("age"), String::from("26"));

    let name = map.get("name");
    let age = map.get("age");

    // println!("{:?}", name.unwrap());
    println!("Name {}", name.unwrap());
    println!("Age {}", age.unwrap());
}

#[test]
fn btree_map() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("Ricid"));
    map.insert(String::from("age"), String::from("26"));
    map.insert(String::from("country"), String::from("Indonesia"));

    for entry in map {
        println!("{} : {}", entry.0, entry.1);
    }
}

#[test]
fn hash_set() {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("Ricid")); 
    set.insert(String::from("Ricid")); 
    set.insert(String::from("Kumbara")); 
    set.insert(String::from("Kumbara")); 
    set.insert(String::from("Kagenou")); 
    set.insert(String::from("Kagenou")); 

    for v in set {
        println!("{}", v);
    }
}

#[test]
fn btree_set() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    set.insert(String::from("Ricid")); 
    set.insert(String::from("Ricid")); 
    set.insert(String::from("Kumbara")); 
    set.insert(String::from("Kumbara")); 
    set.insert(String::from("Kagenou")); 
    set.insert(String::from("Kagenou")); 

    for v in set {
        println!("{}", v);
    }
}

#[test]
fn iterator_test() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    println!("{:?}", iterator);

    while let Some(v) = iterator.next() {
        println!("{}", v);
    }

    // for v in iterator {
    //     println!("{}", v);
    // }
    println!("{:?}", iterator);
}

#[test]
fn iterator_method() {
    let vector = vec![1, 2, 3, 4, 5];

    let sum: i32 = vector.iter().sum();
    println!("{}", sum);
    
    let count: usize = vector.iter().count();
    println!("{}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
    
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

#[allow(dead_code)]
fn connect_database(host: Option<String>) {
    match host {
        None => {
            panic!("No database host provided");
        }
        Some(host) => {
            println!("Connecting to database {}", host);
        }
    }
}

#[test]
fn panic_test() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None);
}

#[allow(dead_code)]
fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

#[test]
fn recoverable_error() {
    let cache = connect_cache(Some("localhost".to_string()));
    // let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Connected to {}", host);
        }
        Err(err) => {
            println!("Error with message {}", err);
        }
    }
}

#[allow(dead_code)]
fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            Err("No email host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

#[allow(dead_code)]
fn connect_application(host: Option<String>) -> Result<String, String> {
    connect_cache(host.clone())?;
    connect_email(host.clone())?;

    Ok("Connected to application".to_string())
}

#[test]
fn application_error() {
    // let result = connect_application(Some("localhost".to_string()));
    let result = connect_application(None);

    match result {
        Ok(host) => {
            println!("Success: {}", host);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn lifetime_annotation() {
    let value1 = "Ricid";
    let value2 = "Kumbara";
    let result = longest(value1, value2);

    println!("{}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn lifetime_struct() {
    let student: Student = Student {
        name: "Ricidddd",
        last_name: "Kumbara"
    };

    println!("{}", student.name);
    println!("{}", student.last_name);

    let student2: Student = Student {
        name: "Fulan",
        last_name: "A"
    };

    let result = longest_student_name(&student, &student2);
    println!("{}", result);
    
    let result = student.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, ID> where ID: Ord {
    id: ID,
    name: &'a str,
}

#[test]
fn liftime_annotation_generic() {
    let teacher: Teacher<i32> = Teacher { 
        id: 10, 
        name: "Ricid", 
    };
    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn attribute_derive() {
    let company = Company {
        name: "Ricid Kumbara".to_string(),
        location: "Indonesia".to_string(),
        website: "https://motivatorbugs.com".to_string(),
    };

    let company2 = Company {
        name: "Ricid Kumbara".to_string(),
        location: "Indonesia".to_string(),
        website: "https://motivatorbugs.com".to_string(),
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[test]
fn smart_pointer_box() {
    let value: Box<i32> = Box::new(10);
    println!("{}", value);
    
    display_number(*value);
    display_number_reference(&value);

    println!("{}", value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End
}

#[test]
fn box_enum() {
    let category: ProductCategory = ProductCategory::Of (
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End)
        )),
    );
    println!("{:?}", category);
    print_category(&category);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}

#[test]
fn dereference() {
    let value1: Box<i32> = Box::new(10);
    let value2: Box<i32> = Box::new(2);

    let result: i32 = *value1 * *value2;
    print!("{}", result);
}

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }   
}

#[test]
fn dereference_struct() {
    let value = MyValue {
        value: 10,
    };

    println!("{}", *value);
}

fn say_hello_reference(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn deref_reference() {
    let name = MyValue {
        value: "Ricid".to_string(),
    };

    say_hello_reference(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Droppping book: {}", self.title)
    }
}

#[test]
fn cleanup_drop() {
    let book = Book{
        title: "Rust Programming".to_string(),
    };

    println!("{}", book.title);
}

enum Brand {
    Of(String, Rc<Brand>),
    End
}

#[test]
fn multiple_ownership() {
    // let apple = ProductCategory::Of("Apple".to_string(), Box::new(ProductCategory::End));
    // let laptop = ProductCategory::Of("Apple".to_string(), Box::new(apple));
    // let smartphone = ProductCategory::Of("Apple".to_string(), Box::new(apple));

    let apple: Rc<Brand> = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count {}", Rc::strong_count(&apple));

    let laptop: Brand = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count {}", Rc::strong_count(&apple));

    {
        let smartphone: Brand = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count {}", Rc::strong_count(&apple));
    }

    println!("Apple reference count {}", Rc::strong_count(&apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn interior_mutability_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Ricid".to_string()),
        active: RefCell::new(true),
    };

    {
        let mut result: std::cell::RefMut<String> = seller.name.borrow_mut();
        *result = "Kumbara".to_string();
    }

    println!("{:?}", seller);
}