use kratos::{print, println};

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}


#[test_case]
fn test() {
    print!("Test 1...");
    assert!(true);
    println!("[Ok]");
}
