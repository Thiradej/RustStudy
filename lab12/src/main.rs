//1
use std::env::args;
use std::process::exit;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Error jujuuuuu");
        exit(1);
    }
    let file = &args[1];

    let con = match fs::read_to_string(file) {
        Ok(n) => n,
        Err(_) => {
            println!("Error 55");
            exit(1)
        }
    };

    let line = con.lines().count();
    let words = con.split_whitespace().count();
    let characters = con.chars().count();

    println!("File: {}", con);
    println!("Lines: {}", line);
    println!("Words: {}", words);
    println!("Characters: {}", characters);
}


//2
// use std::env::args;
// use std::process::exit;
// use std::error::Error;

// fn main() {
//     let args: Vec<String> = args().collect();

//     if args.len() != 4 {
//         println!("Error na ja", );
//         exit(1)
//     }else{
//         let num1: f64 =  match args[1].parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 println!("Error {} is not valid number.", args[1]);
//                 exit(1)
//             }
//         };
//         let op = &args[2];
//         let num2: f64 = match args[3].parse() {
//             Ok(n) => n,
//             Err(_) => {
//                 println!("Error {} is not valig number.", args[3]);
//                 exit(1)
//             }
//         };
//         let result = match op.as_str() {
//             "+" => {num1 + num2},
//             "-" => {num1 + num2},
//             "*" => {num1 + num2},
//             "/" => {
//                 if num2 == 0.0 {
//                     println!("Error TT");
//                     exit(1)
//                 }else {num1 / num2}
//             },
//             _ => {
//                 println!("Error ek leaw");
//                 exit(1)
//             }
//         };
//         println!("{}", result);
//     }
// }



//3
// use std::env::args;
// use std::process::exit;
// use std::fs;

// fn main() {
//     let args: Vec<String> = args().collect();

//     if args.len() != 2 {
//         println!("Error jujuuuuu");
//         exit(1)
//     }
//     let file = &args[1];

//     let con = match fs::read_to_string(file) {
//         Ok(n) => n,
//         Err(_) => {
//             println!("Error 55");
//             exit(1)
//         }
//     };

//     let line = con.lines().count();
//     let words = con.split_whitespace().count();
//     let characters = con.chars().count();

//     println!("File: {}", file);
//     println!("Lines: {}", line);
//     println!("Words: {}", words);
//     println!("Characters: {}", characters);
// }



//4
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read};

// fn main() -> io::Result<()> {
//     let mut con = String::new();
//     con.push_str(std::env::consts::OS);
//     con.push_str(&std::env::var("USERNAME").unwrap());
//     loop {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("Err");
//         con += input.as_str();
//         if input.trim() == "".to_string() {
//             break
//         }
//     }   
//     std::fs::write("ok.txt", con);
//     let file = File::open("ok.txt")?;
//     let reader = BufReader::new(file);

//     for line_result in reader.lines() {
//         let line = line_result?;
//         for word in line.split_whitespace() {
//             println!("{}", word.to_uppercase());
//         }
//     }
//     Ok(())
// }