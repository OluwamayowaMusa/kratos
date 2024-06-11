use kratos::{print, println};

#[test_case]
fn test() {
    print!("Test 1...");
    assert!(true);
    println!("[Ok]");
}
