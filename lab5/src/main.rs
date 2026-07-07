use::std::{io, io::Write};

fn cel_to_fahren (c: f64) -> f64 {
    c * 9.0/5.0 + 32.0
}

fn fahren_to_cel (f: f64) -> f64 {
    ( f - 32.0 ) * 5.0/9.0
}

fn main() {
  m();
}

fn s() {

println!("Enter 1 : Cel -> Fahren");
        println!("Enter 2 : Fahren -> Cel");
    
        println!("Enter your choice: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");


        match choice.trim() {
        "1" => {
            print!("Enter cel : ");
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let c: f64= input.trim().parse().unwrap();
            println!("Fahren : {}", cel_to_fahren(c))

            }
        "2" => {
            print!("Enter Fah : ");
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let f: f64= input.trim().parse().unwrap();
            println!("Fahren : {}", fahren_to_cel(f))

            }
        _ => {
            println!("Invlid");
            
        }
        
    }
}

//locate//
// n = เเถว  ,  m = ลำดับในเเถว//
fn pascal(n: u64, m: u64) -> u64 {
    if m == 0 || m == n {
        1
    }else{
        pascal(n - 1, m - 1) + pascal( n - 1, m)
    }
}

fn m() {
loop {
    //input//
    let mut input = String::new();
    print!("Enter number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let rows: u64= input.trim().parse().unwrap(); //rows change U32


    if rows > 9 {
        println!("Error");
        break;
    }
    //println!("Pascal : {} ", rows);

    //loop for rows
    for i in 0..rows{

        //space between a number
        for _ in 0..(rows - i - 1)  {
            print!("  ");
        }
        
        //loop in side rows and call pascal
        for j in 0..=i{
            print!("{:4}", pascal(i, j));
        }
        println!();
        
    }break;
}
}