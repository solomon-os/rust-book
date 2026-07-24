unsafe extern "C" {
    safe fn abs(x: i32) -> i32;
}

static mut COUNTER: u32 = 20;

pub fn snippets() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x01234usize;
    let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10) }; // will fail
    // dbg!(values);

    println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        COUNTER += 10;
        COUNTER += 10;
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
