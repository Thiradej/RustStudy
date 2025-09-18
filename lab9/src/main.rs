//1

// use std::mem;
// use std::num::NonZeroUsize;
// #[derive(Debug)]

// struct S1{
//     a: u8,
//     b: u64,
//     c: u8,
// }

// #[repr(C)] struct S2{
//     a: u8,
//     b: u64,
//     c: u8,
// }

// struct S3{
//     b: u64,
//     c: u8,
//     a: u8,
// }

// fn main() {
//     println!("S1 size = {} align = {}", size_of::<S1>(), align_of::<S1>());
//     println!("S2 size = {} align = {}", size_of::<S2>(), align_of::<S2>());
//     println!("S3 size = {} align = {}", size_of::<S3>(), align_of::<S3>());
// }


//2
// use std::num::NoneZeroUsize;
// fn main() {
//     println!("&u8 = {} Option<&u8> = {}", size_of::<&u8>(), size_of::<Option<&u8>>());
//     println!("usize = {} Option<usize> = {}", size_of::<usize>(), size_of::<Option<usize>>());
//     println!("NonZeroUsize = {} Option<NonZeroUsize> = {}", size_of::<NonZeroUsize>(), size_of::<Option<NonZeroUsize>>());
// }


//3

// fn main() {
//     let mut x = Vec::new();
//     for i in 1..=40 {
//         x.push(i);
//     }println!("len = {}  cap = {} ptr = {:p}", x.len(), x.capacity(), x.as_ptr());

//     let mut y = Vec::with_capacity(32);
//     println!("Preallocated first cap = {}", y.capacity());
//     for j in 1..=40 {
//         y.push(j);
//     }

//     println!("after = {}", y.capacity())
// }


//4
#[repr(C)]struct Data {
    i: u32,
    c: char,
    b: bool,
}
fn main() {
    let x: u32 = 0x1122344;
    println!("ne = {:?} be = {:?} le = {:?}", x.to_ne_bytes(), x.to_be_bytes(), x.to_le_bytes());

    let d = Data{i:12, c:'A', b:true};
    let slice: &[u8] = unsafe {
        std::slice::from_raw_parts(&d as *const Data as *const u8, size_of::<Data>())
    };
    println!("Raw bytes: {:?}", slice);
}