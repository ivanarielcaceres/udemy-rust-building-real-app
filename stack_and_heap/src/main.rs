fn main() {
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    // Box is a type of Smart pointer in rust
    // Allocate some memory on the heap for us
    let e = Box::new(7);
    return d + *e;
}
