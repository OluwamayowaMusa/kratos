use alloc::string::String;
use kratos::{print, println};

// Test
mod test_allocator;
mod test_bit_manipulation;
mod test_gdt;

pub struct TestCase {
    pub name: &'static str,
    pub test: &'static (dyn Fn() -> Result<(), String> + Send + Sync),
}

pub fn test_runner(tests: &[&TestCase]) {
    println!("Running {} tests", tests.len());

    for test_case in tests {
        print!("{}... ", test_case.name);
        if let Err(e) = (test_case.test)() {
            println!("{}", e);
        } else {
            println!("[Ok]");
        }
    }
}

#[macro_export]
macro_rules! create_test {
    ($name:ident, $content:block) => {
        paste::paste! {
            #[test_case]
            static $name: TestCase = TestCase {
                name: stringify!($name),
                test: &[<$name _test>],
            };
            fn [<$name _test>]() -> Result<(), alloc::string::String> {
                $content
            }
        }
    };
}

// Implementing macros for test continuity, assert causes panic
#[macro_export]
macro_rules! test_eq {
    ($a:expr, $b:expr) => {
        if $a != $b {
            return Err(alloc::format!(
                "{}:{} {:?} != {:?}",
                file!(),
                line!(),
                $a,
                $b
            ));
        }
    };
}

#[macro_export]
macro_rules! test_ne {
    ($a:expr, $b:expr) => {
        if $a == $b {
            return Err(alloc::format!(
                "{}:{} {:?} != {:?}",
                file!(),
                line!(),
                $a,
                $b
            ));
        }
    };
}

#[macro_export]
macro_rules! test_ge {
    ($a:expr, $b:expr) => {
        if $a < $b {
            return Err(alloc::format!(
                "{}:{} {:?} != {:?}",
                file!(),
                line!(),
                $a,
                $b
            ));
        }
    };
}

#[macro_export]
macro_rules! test_true {
    ($a:expr) => {
        if !$a {
            return Err(alloc::format!(
                "{}:{} {:?} is not true",
                file!(),
                line!(),
                $a,
            ));
        }
    };
}
