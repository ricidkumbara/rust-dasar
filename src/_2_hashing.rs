#[test]
fn test_rust_argon() {
    use argon2::{self, Config};

    let password = b"password";
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    let matches = argon2::verify_encoded(&hash, password).unwrap();

    println!("{:?}", password);
    println!("{:?}", salt);
    println!("{:?}", hash);
    println!("{:?}", matches);
    assert!(matches);
}

#[test]
fn test_bcrypt() {
    extern crate bcrypt;

    use bcrypt::{DEFAULT_COST, hash, verify};

    let hashed = hash("hunter2", DEFAULT_COST);
    println!("{:?}", hashed);

    let valid = verify("hunter2", &hashed.unwrap());
    println!("{:?}", valid);
}