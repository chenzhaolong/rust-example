fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    let s1 = s;
    println!("{}", s1);

    let a: i32 = 10;
    let b = a;
    println!("{}", a);

    struct A {
        data: i32
    }

    impl Clone for A {
        fn clone (&self) -> A {
         A {data: self.data}
        }
    }

    impl Copy for A {};

    let a1 = A { data: 12 };

    let a2 = a1;

    println!("{}", a2.data);
}
