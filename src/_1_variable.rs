#[test]
fn test_variable() {
    let a: i32 = 10;
    println!("{}", a);
}

#[test]
fn test_number() {
    // Number merupakan tipe data yang disimpan di heap, jika diassign kevariable lain,
    // maka value akan dicopy (ownership tidak akan dipindahkan)
    let a: i32 = 10;
    let b: i32 = a;
    
    println!("{}", a);
    println!("{}", b);
}

#[test]
fn test_string() {
    // String merupakan tipe data yang disimpan di heap, ownership akan dicopy jika variable 
    // diassign ke variable lain
    // solusinya bisa menggunakan fungsi clone()
    let a: String = "Ricid".to_string();
    let b: String = a;

    // println!("{}", a); // Error karena ownership dipindahkan
    println!("{}", b);

    let c: String = "Ricid".to_string();
    let d: String = c.clone();

    println!("{}", c);
    println!("{}", d);
}

#[test]
fn test_string_mutable() {
    let mut a: String = "Ricid".to_string();
    let mut b: String = a.clone();

    println!("{}", a);
    println!("{}", b);

    a = "Ricid_".to_string();
    b = "Kumbara_".to_string();

    println!("{}", a);
    println!("{}", b);

    a.push_str(" Kumbara".to_string().as_str());
    b.push_str(" Kumbara");

    println!("{}", a);
    println!("{}", b);
}