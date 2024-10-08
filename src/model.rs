#[allow(dead_code)]
pub struct User {
    pub first_name: String,
    pub username: String,
    pub email: String,
    pub age: u8,
}

#[allow(dead_code)]
impl User {
    pub fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}