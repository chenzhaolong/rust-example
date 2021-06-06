fn main() {
    let a = B {b1: 10};
    println!("{}", a.go());
    println!("{}", B::doing(10));

    let c = C {c1: -1};
    println!("{}", c.sweet());
    println!("{}", C::fun(12.0));
    println!("{}", c.hi(c.c1 as f32));
}

trait A {
    fn go (self) -> i32;

    fn doing (a: i32) -> bool;
}

struct B {
    b1: i32
}

impl A for B {
    fn go (self) -> i32 {
        return self.b1 + 10
    }

    fn doing (a: i32) -> bool {
        return a > 10
    }
}

struct C {
    c1: i32
}

impl C {
    fn sweet (self) -> bool {
        return self.c1 > 0;
    }

    // fn hi (self: f32) -> bool {
    //     return self.c1 > 10.0;
    // }

    fn fun (a: f32) -> bool {
        return a > 10.0;
    }
}