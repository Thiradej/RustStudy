use std::io;

fn main() {
    no1();
}

fn no1() {
    let brick = "*";

    //Input player 1
    let mut p1 = String::new();
    println!("Player 1:");
    io::stdin().read_line(&mut p1).expect("Failed to read");
    let name1 = p1.trim().to_string();

    //Input player 2
    let mut p2 = String::new();
    println!("Player 2:");
    io::stdin().read_line(&mut p2).expect("Failed to read");
    let name2 = p2.trim().to_string();
    
    //checks length of longer name
    let long;
    let short;
    let diff;
    let horilen ;
    if name1.len() > name2.len() {
         long = name1.len();
         short = name2.len();
    } else {
         short = name1.len();
         long = name2.len();
    }
    diff = long - short;
    horilen = 25 + short + long;

    println!("{}", long);
    println!("{}", short);
    println!("{}", diff);

    //vertical line
    for line in 1..=9 {
        if line == 1 || line == 5 || line == 9 {
            let mut i = 0;
            while i < 13 + long {
                print!("{}", brick);
                i += 1;
            }
            println!();
        } else if line == 3 {
            print!("{}", brick);
            if name1.len() > name2.len() {
                print!(" player 1:{name1} "); 
                println!("{}", brick);
            } else {
                print!(" player 1:{name1} ");
                let mut i = 0;
                while i < diff {
                    print!(" ");
                    i += 1;
                }
                println!("{}", brick)
            }
            
        } else if line == 7 {
            print!("{}", brick);
            if name2.len() > name1.len() {
                print!(" player 2:{name2} "); 
                println!("{}", brick);
            } else {
                print!(" player 2:{name2} ");
                let mut i = 0;
                while i < diff {
                    print!(" ");
                    i += 1;
                }
                println!("{}", brick)
            }
        } else {
            print!("{}", brick);
            let mut i = 0;
            while i < 11 + long {
                print!(" ");
                i += 1;
            } 
            println!("{}", brick);
        }
    }

    println!();

    //horizontal
    for line in 1..=5 {
        if line == 1 || line == 5 {
            let mut i = 0;
            while i < horilen {
                print!("{}", brick);
                i += 1;
            }
            println!();
        } else if line == 3 {
            print!("{}", brick);
            print!(" player 1:{name1} ");
            print!("{}", brick);
            print!(" player 2:{name2} ");
            print!("{}", brick);
            println!();
        } else {
            print!("{}", brick);
            let mut i = 0;
            while i < 11 + name1.len() {
                print!(" ");
                i += 1;
            } 
            print!("{}", brick);
            let mut i = 0;
            while i < 11 + name2.len() {
                print!(" ");
                i += 1;
            } 
            print!("{}", brick);
            println!();
        }
    }

}



fn no2() {
    let mut input = String::new();

    let mut employees: [(String,u32,f32);5] = [(String::new(), 0 , 0.0),(String::new(), 0 , 0.0),(String::new(), 0 , 0.0),(String::new(), 0 , 0.0),(String::new(), 0 , 0.0)];
    for i in 0..5{
        let emp = i + 1;
        println!("Enter Employee's {}'s Name: ", emp);
        io::stdin().read_line(&mut input).expect("Failed to read");
        employees[i].0 = input.trim().to_string();
        input.clear();
        println!("Enter Employee's {}'s Age: ", emp);
        io::stdin().read_line(&mut input).expect("Failed to read");
        employees[i].1 = input.trim().parse().expect("Invalid Age");
        input.clear();
        println!("Enter Employee's {}'s Salary: ", emp);
        io::stdin().read_line(&mut input).expect("Failed to read");
        employees[i].2 = input.trim().parse().expect("Invalid Salary");
        input.clear();
    }
    let mut highest:f32 = 0.0;
    let mut old = 0;
    let mut pay:f32 = 0.0;
    for i in 0..5{
        if highest < employees[i].2{
            highest = employees[i].2;
        }
        pay += employees[i].2;
        if old < employees[i].1{
            old = employees[i].1;
        }
        println!("Employee name = {:?}, Age = {:?}, Salary = {:?}", employees[i].0, employees[i].1,  employees[i].2);
    }
        for i in 0..5{
            if employees[i].2 == highest{
                println!("Employee with the highest salary is: {:?} with a salary of {:?}", employees[i].0 , employees[i].2);
            } 
            if employees[i].1 == old{
                println!("Oldest employee is: {:?} with an age of {:?}", employees[i].0 , employees[i].1);

            }

        }


}