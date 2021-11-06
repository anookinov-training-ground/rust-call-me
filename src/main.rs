fn main() {
    println!("Hello, world!");
    let x = bar; // function item
    let mut x = bar_generic::<i32>; // function item
    println!("{}", std::mem::size_of_val(&x));
    // x = bar_generic()::<u32>;
    baz(bar_baz::<u32>);
    baz(bar_baz::<i32>);
}

fn bar() {}

fn bar_generic<T>() {}

fn bar_baz<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}
