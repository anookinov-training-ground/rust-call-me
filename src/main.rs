fn main() {
    println!("Hello, world!");
    let x = bar; // function item
    let mut x = bar_generic::<i32>; // function item
    println!("{}", std::mem::size_of_val(&x));
    // x = bar_generic()::<u32>;
    baz(bar_generic::<u32>);
    baz(bar_generic::<i32>);
    baz_u32(bar_baz::<u32>);
    baz_u32(bar_baz::<i32>);
    quox(bar_generic::<u32>);
}

fn bar() {}

fn bar_generic<T>() {}

fn bar_baz<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}

fn baz_u32(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}

impl<F> FnOnce() for F
where
    F: Fn(),
{
    fn call(&self) {
        Fn::call(&self)
    }
}

impl<F> FnOnce() for F
where
    F: FnMut(),
{
    fn call(mut self) {
        FnMut::call(&mut self)
    }
}

impl<F> FnMut() for F
where
    F: Fn(),
{
    fn call(&mut self) {
        Fn::call(&*self)
    }
}

fn quox<F>(f: &F)
where
    F: Fn(),
{
    (f)()
}

fn quox_fnmut<F>(f: &mut F)
where
    F: FnMut(),
{
    (f)()
}

fn quox_fnonce<F>(f: F)
where
    F: FnOnce(),
{
    (f)()
}
