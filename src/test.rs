use alloc::string::String;

use kratos::{print, println};

pub struct TestCase {
    pub name: &'static str,
    pub test: &'static (dyn Fn() -> Result<(), String>)
}


pub fn test_runner(tests: &[&TestCase]) {
    println!("Running {} tests", tests.len());

    for test_case in tests {
        println!("{}.. .", test_case.name);
        if let Err(e) = (test_case.test)() {
            println!("{}", e);
        } else {
            println!("[Ok]");
        }
    }
}
