use std::io;

fn main() {
    q2();
}


fn q1() {

    //1
    let mut scores   = vec![85, 92, 78, 96, 88, 73, 91, 84];
    scores.sort();
    println!("{:?}", scores);
    println!("There are : {}", scores.len());
    
     //2
    
    println!("{:?}", scores.iter().max());
    println!("{:?}", scores.iter().min());
    if scores.contains(&90) {
        println!("Good job")
    }else {
        println!("Noob")
    }

    //3
    scores.push(87);
    scores.sort();
    let re = scores.pop();
    println!("Update : {:?}", scores);

    //4
    let count = scores.iter().count();
    println!("{}", count);

    let count = scores.iter().filter(|x| x >= &&85).count();
    println!("Scores >= 85: {}", count);

    
    let above_80: Vec<i32> = scores.iter().filter(|x|x > &&80).map(|x| x).collect();
    println!("Scores > 80: {:?}", above_80);

    
    scores.retain(|x|x >= &75);
    println!("Removed scores < 75: {:?}", scores);
}



fn q2() {
    let mut warehouse: Vec<(u32, String, u32)> = Vec::new();

    loop {
        println!("Warehouse Inventory Management:");
        println!("1. Add New Product");
        println!("2. Update Stock Quantity");
        println!("3. Remove Product");
        println!("4. List All Products");
        println!("5. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");
        let choice = choice.trim();


        //choice 1 add pro have input/
        if choice == "1" {
            //  Add Product
            println!("Enter product ID: ");
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).expect("Failed to read ID");
            let id: u32 = match id_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid ID.");
                    continue;
                }
            };

            //Check if ID exists (Hints)
            let mut id_exists = false;
            for (existing_id, _, _) in &warehouse {
                if *existing_id == id {
                    id_exists = true;
                    break;
                }
            }

            if id_exists {
                println!("Product with ID {} already exists!", id);
                continue;
            }

            println!("Enter product name: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed to read name");
            let name = name.trim().to_string();

            println!("Enter quantity: ");
            let mut qty_input = String::new();
            io::stdin().read_line(&mut qty_input).expect("Failed to read quantity");
            let quantity: u32 = match qty_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid quantity.");
                    continue;
                }
            };

            warehouse.push((id, name, quantity));
            println!("Product added!");


            //choice 2 have input/ check num and !num /
        } else if choice == "2" {
            //Update quantity
            println!("Enter product ID to update: ");
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).expect("Failed to read ID");
            let id: u32 = match id_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid ID.");
                    continue;
                }
            };

            let mut found = false;
            for item in warehouse.iter_mut() {
                if item.0 == id {
                    println!("Enter new quantity: ");
                    let mut qty_input = String::new();
                    io::stdin().read_line(&mut qty_input).expect("Failed to read quantity");
                    let new_qty: u32 = match qty_input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid quantity.");
                            continue;
                        }
                    };

                    item.2 = new_qty;
                    println!("Quantity updated.");
                    found = true;
                    break;
                }
            }

            if !found {
                println!("Product ID not found.");
            }
            
            
            //choice 3 remove pro
        } else if choice == "3" {
            // Remove product
            println!("Enter product ID to remove: ");
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).expect("Failed to read ID");
            let id: u32 = match id_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid ID.");
                    continue;
                }
            };

            let index = warehouse.iter().position(|(pid, _, _)| *pid == id);
            if let Some(i) = index {
                warehouse.remove(i);
                println!("Product removed.");
            } else {
                println!("Product not found.");
            }


            //choice 4 check have item in stock
        } else if choice == "4" {
            //List all
            if warehouse.is_empty() {
                println!("Inventory is empty.");
            } else {
                println!("Inventory List:");
                for (id, name, qty) in &warehouse {
                    println!("ID: {}, Name: {}, Quantity: {}", id, name, qty);
                }
            }

        } else if choice == "5" {
            //Exit
            println!("Exiting program.");
            break;
        } else {
            println!("Invalid choice.");
        }
    }
}


