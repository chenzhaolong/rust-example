fn main() {
    // knowArray();
    // limitArr();
    getStr();
}

fn knowArray () {
    // 创建数组
    let mut a: [i32; 3] = [1,2,3];
    println!("first: {}", a[1]);
    // 数组比较
    let b: [i32; 3] = [1,2, 3];
    println!("second: {}", b == a);
    // 获取数组的值
    println!("third {}", b[0] + a[1]);
    let c: [i32; 3] = [1,2,5];
    a = c;
    for i in &a {
        println!("{}", i)
    }
    let s: &mut [i32; 3] = &mut a;
    Slice(s);
    println!("s:{}", s[2]);
    println!("a:{}", a[2]);
}

fn Slice(a: &mut [i32]) {
    a[2] = 5;
    println!("slice{}", a[2])
}

fn Fat(a: &[i32]) {
    println!("Fat{}", a[1]);
}

fn limitArr() {
    // [0, 10)
    let a = 1..10;
    for i in a {
        println!("i:{}", i);
    }
    // [1, 10]
    let b = 1..=10;
    for j in b {
        println!("j:{}", j);
    }
    // a的子集3-7
    let full: [i32; 6] = [1,2,3,4,5,6];
    let c = &full[..6];
    for z in c {
        println!("z:{}", z);
    }
}

fn getStr() {
    let a: &str = "aaa";
    println!("{}", a);
    println!("len:{}", a.len());
    println!("len:{}", &a[1..]);

    let mut s = String::from("hello");
    s.push_str(" nihao");
    println!("s: {}", s);
}