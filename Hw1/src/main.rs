use::std::io::Write;
use::std::{io, self};


fn main() {

    loop {
        
        //input temp
        print!("Enter current temperature (Celsius) or 'stop' to exit: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        
        
        //input stop
        if input == "stop" {
            println!("Exiting temperature regulation system.");
            break;
        }
        
        //condition
        match input.parse::<i32>() {
    Ok(temp) => {
        if temp <= 18 {
            println!("Heater turned on.");
        } else if temp >= 24 {
            println!("Air conditioner turned on.");
        } else if temp == 20 {
            println!("Heater turned off.");
        }else if temp == 22 {
            println!("Air conditioner turned off.");
        }


        println!("Wating 5 minutes...");
        

        }Err(_) => {
            println!("Invalid input, please enter a valid number or 'stop'");
        }
    }  
    
}
}
