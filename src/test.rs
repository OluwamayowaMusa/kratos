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

macro_rules! create_test {
    ($name:ident, $content:block) => {
        paste::paste! {
            #[test_case]
            static $name: TestCase = Testcase {
                name: stringify!($name),
                test: &[<$name _test>],
            };
            fn [<$name _test>]() -> Result<(), alloc::string::String> {
                $content
            }
        }
    };
}
