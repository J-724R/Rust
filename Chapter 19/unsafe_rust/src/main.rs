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
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    split_at_mut(&mut vec![1, 2, 3, 4, 5, 6], 3);

    println!("Using extern Functions to Call External Code");

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("Calling Rust Functions from Other Languages");
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    println!("Accessing or Modifying a Mutable Static Variable");
    println!("name is: {HELLO_WORLD}");
}

static HELLO_WORLD: &str = "Hello, world!";

extern "C" {
    fn abs(input: i32) -> i32;
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
