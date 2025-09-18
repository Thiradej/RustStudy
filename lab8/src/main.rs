//1
use std::mem;

// fn fac(n: u32) -> u32 {
//     println!("Calculating Factorial({})", n);
//     println!("Value: {}, Memory Address: {:p}, Size: {}", n, &n, mem::size_of_val(&n));

//     if n <= 1 {
//         1
//     }else {
//         n * fac(n-1)
//     }
    
// }

// fn main() {
//     let x = fac(5) ;
//     println!("Something : {}", x)
// }

//2

// static A: u32 = 3;
// static B: i32 = -1_000_000;

// fn s() {
//     println!("Static A: {}, Adress: {:p}", A, &A);
//     println!("Static B: {}, Adress: {:p}", B, &B);
// }
// fn main() {
//     println!("Static A: {}, Adress: {:p}", A, &A);
//     println!("Static B: {}, Adress: {:p}", B, &B);
//     println!("From another function:");
//     s();
// }



// 3

#[derive(Debug)]

struct Point {
    x: f64,
    y: f64,
}


fn main() {
    let p1 = Point {x: 15.6, y: 26.7};
    println!("Point on stack size: {} bytes", mem::size_of_val(&p1));

    let p2 = Box::new(p1);
    println!("Boxed point on stack size: {} bytes (pointer)", mem::size_of_val(&p2));
    println!("Dereferenced boxed point: {:?}", &p2);

    let p3 = Box::new(p2);
    println!("Double boxed point size: {} bytes", mem::size_of_val(&p3));

}