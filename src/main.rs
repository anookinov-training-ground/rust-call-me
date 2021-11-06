#![feature(const_trait_impl, const_fn_trait_bound)]

use std::future::Future;

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
    quox(&mut bar_generic::<u32>);

    let f = |x: i32, y: i32| x + y; // non-capturing closure
    let f = || ();
    baz(f);
    quox(&f);

    let z = String::new();
    let f = || {
        // let _ = z;
        println!("{}", z);
    };
    // struct FClosure<'scope> {
    //     z: &'scope String,
    // }
    // impl<'scope> Fn() for FClosure<'scope> {
    //     fn call(&self) {
    //         // copy-paste from closure definition
    //         println!("{}", self.z);
    //     }
    // }

    let mut z = String::new();
    let f = || {
        z.clear();
    };
    // struct FClosure<'scope> {
    //     z: &'scope mut String,
    // }
    // impl<'scope> FnMut() for FClosure<'scope> {
    //     fn call(&mut self) {
    //         // copy-paste from closure definition
    //         self.z.clear();
    //         // & &mut String -> &String
    //     }
    // }

    // baz(f);
    quox_fnmut(f);

    let mut z = String::new();
    let f = || {
        drop(z);
    };
    // struct FClosure<'scope> {
    //     z: String,
    // }
    // impl<'scope> FnOnce() for FClosure<'scope> {
    //     fn call(self) {
    //         // copy-paste from closure definition
    //         drop(self.z);
    //     }
    // }

    // baz(f);
    quox_fnonce(f);

    let mut z = String::new();
    let f = move || {
        println!("{}", z);
        // z is dropped here
    };
    // struct FClosure<'scope> {
    //     z: String,
    // }
    // impl<'scope> FnOnce() for FClosure<'scope> {
    //     fn call(self) {
    //         // copy-paste from closure definition
    //         drop(self.z);
    //     }
    // }

    let f: &dyn Fn() = &f;
    quox_fnmut(f);

    let f: &mut dyn FnMut() = &f;
    quox_fnmut(f);

    let f: Box<dyn FnOnce()> = Box::new(f);
    quox_fnonce(f);

    let x = || {
        String::from("foo");
    };
    foo(x);

    // tokio::spawn(quox_lt(|x| x)); // + 'static
    quox_lt(|x| x).await;
}

// async fn quox_lt<F>(f: F)
fn quox_lt<F>(f: F) -> impl Future<Output = ()>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    async move {
        let _ = f;
    }
}

const fn test_foo() {
    let x = || {
        String::from("foo");
    };
    foo(x);
}

const fn make_zero() -> i32 {
    0
}

const fn foo<F: ~const FnOnce()>(f: F) {
    f()
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

// impl<F> FnOnce() for F
// where
//     F: Fn(),
// {
//     fn call(&self) {
//         Fn::call(&self)
//     }
// }

// impl<F> FnOnce() for F
// where
//     F: FnMut(),
// {
//     fn call(mut self) {
//         FnMut::call(&mut self)
//     }
// }

// impl<F> FnMut() for F
// where
//     F: Fn(),
// {
//     fn call(&mut self) {
//         Fn::call(&*self)
//     }
// }

fn quox<F>(f: &F)
where
    F: Fn(),
{
    (f)()
}

// fn quox_fnmut<F>(f: &mut F)
fn quox_fnmut<F>(mut f: F)
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

// Box<dyn Fn()> did not implement Fn()

// impl FnOnce() for Box<dyn FnOnce()> {
//     fn call(self) {
//         let x: dyn FnOnce() = self.0;
//         x.call()
//     }
// }

fn hello(f: Box<dyn Fn()>) {
    f()
}

fn hello_fnmut(mut f: Box<dyn FnMut()>) {
    f()
}

fn hello_fnonce(mut f: Box<dyn FnOnce()>) {
    f()
}

pub fn make_fn() -> impl Fn() {
    // let x = String::new();
    let z = String::new();
    // let x2 = &x;
    // let z: &'a String = &z;
    {
        // let x = &x;
        move || {
            // println!("{}", x);
            // println!("{}", x2);
            println!("{}", z);
            // drop(z);
        }
    }
}
