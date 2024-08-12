use crate::third::say_hello as say_hello_third;

pub fn say_hello() {
    println!("Hello from first module");

    say_hello_third();
}

pub mod first_second {
    pub mod first_third {
        pub fn say_hello() {
            // bisa spt ini
            crate::first::say_hello();

            // bisa spt ini
            super::super::say_hello();
        }
    }
}