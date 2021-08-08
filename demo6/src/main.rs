fn main() {
    f1();
    f2(Type::SOURTH);
    let obj = T3 { a: 1, b: 2, c: 3, d: 4, e: 5 };
    let a = T4 (120, 11, 10);
    f3(obj, a);
}

struct T1 (i32, f32);

struct T2 {
    a1: i32,
    a2: T1
}

fn f1() {
    let t1 = T2 {
        a1: 39,
        a2: T1 (45, 90.0)
    };
    let T2 {
        a1: v1,
        a2: T1 (v2, v3)
    } = t1;
    println!("v1: {}", v1);
    println!("v2: {}", v2);
    println!("v3: {}", v3);
}

enum Type {EAST, NORTH, SOURTH, WEST}

fn f2(x: Type) {
    let a: &str;
    match x {
        Type::EAST => {
            a = "1";
        }
        Type::NORTH => {
            a = "2";
        }
        Type::SOURTH => {
            a = "3";
        }
        _ => {
            a = "sadf";
        }
    }
    println!("a {}", a);
}

struct T3 {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
}

struct T4 (i32, i32, i32);

fn f3 (obj: T3, obj1: T4) {
    let T3 { a, ..} = obj;
    let T4 (a1, _, c, ..) = obj1;
    println!("值:{}", a);
    println!("值1:{}", c);
    match a1 {
        1 ..= 10 => println!("{}", "ni"),
        i @ 11 ..= 20 => println!("{}", i),
        i if i > 20 => println!("{}", "192"),
        _ => println!("end"),
    }
}