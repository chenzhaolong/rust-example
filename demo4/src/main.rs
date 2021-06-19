fn main() {
    let a = B {b1: 10};
    println!("{}", a.go());
    println!("{}", B::doing(10));

    let c = C {c1: -1};
    println!("{}", c.sweet());
    println!("{}", C::fun(12.0));
    // println!("{}", c.hi(c.c1 as f32));

    food();
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

trait Food {
    fn get_food (self: &Self) -> i32;

    fn add_food (self: &Self) -> bool;

    fn mul_food (this: &Self) -> i32;
}

fn food () {
    struct Nums (i32, i32);
    impl Food for Nums {
        fn get_food (&self) -> i32 {
            return self.0;
        }
        fn add_food (&self) -> bool {
            return self.1 > 10;
        }
        fn mul_food (this: &Self) -> i32 {
            return this.1;
        }
    }
    // impl Nums {
    //     fn mulFood (self, a: i32) -> bool {
    //         return self.1 + a > 20;
    //     }
    // }
    let inst = Nums (12, 9);
    // print!("getFood {}", inst.get_food());
    // print!("addFood {}", inst.add_food());
    print!("addFood {}", Nums::mul_food(&inst));
    // print!("addFood {}", inst.mulFood(100));

    struct Hello {
        a: i32,
        b: f32,
    }
    impl Food for Hello {
        fn get_food(&self) -> i32 {
            let b = self.b as i32;
            return self.a + b;
        }
        fn add_food(&self) -> bool {
            return self.b > 10.0;
        }
        fn mul_food (this: &Self) -> i32 {
            return this.a;
        }
    }
    let inst1 = Hello {a: 10, b: 13.0};
    print!("A {}", inst1.get_food());
    print!("B {}", inst1.add_food());
}

fn food1 () {    
    struct Hello {
        a: i32,
        b: f32,
    }
    impl Hello {
        fn get_food(&self) -> i32 {
            let b = self.b as i32;
            return self.a + b;
        }
    }
    let inst1 = Hello {a: 10, b: 13.0};
    print!("A {}", inst1.get_food());
}