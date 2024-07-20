fn main() {
    println!("To switch to unsafe Rust, use the unsafe keyword");

    // Dereferencing a Raw Pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("Calling an Unsafe Function or Method");

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    println!("Creating a Safe Abstraction over Unsafe Code");
}
