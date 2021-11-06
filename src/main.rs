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
