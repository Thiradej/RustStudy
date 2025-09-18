fn main() {
//part 1
//1
        //declare
    let year = 2025;
    let mut month = 7;
    
    month = 12;

    println!("Year = {} Month = {}", year, month);

    let mut year = year;
    year = 2026;
    println!("Updated year {}", year);

//2

    let mut price = 99.99;
    //price = 100;
    price = 100.00;
    //mismatches types floating
    let discount_applied = price < 100.00;
    println!("Discount_applied: {} ", discount_applied);

//3
    let available = true;
    let mut in_stock = false;
    let rating = 4.5;
    
    let mut is_good = available && rating > 4.0 && in_stock;
    println!("{}", is_good);
    //modify in_stock and test again
    in_stock = true;
    let mut is_good = available && rating > 4.0 && in_stock;
    println!("{}", is_good);

    println!("!available: {}", !available);
    println!("available || in_stock: {}", available || in_stock);
    println!("rating < 3.0 || rating > 4.0: {}", rating < 3.0 || rating > 4.0);

//4
        //Declare
    let mut score = 80;
        //reassign
    score = score + 10;

    let score = if score > 85 {"Passed"} else {"Failed"};
    println!("Final score: {} ", score);

//5

    let  mut a = 10;

    //scope
    {let b = "Beam"; println!("{}", b);
    //println!("{}", b);
    //not in scope
    a += 5 ;
    println!("{}", a);
    }
println!("{}", a);
    //shadowing
    let b = a + 10; 
    println!("{}", b);

//part 2

let base_price = 150.0;
let mut discount = 0.0;

let student = true;
let early_bird = false;
let coupon = true;

if student {discount += 0.10};
if early_bird {discount += 0.20};
if coupon {discount += 0.05};

let final_price = base_price * (1.00-discount);
let free_entry = {
    let free_entry = final_price < 50.0;
    free_entry
};


println!("Base ticket price: {}", base_price);
println!("Student discount applied: {}", student);
println!("Early bird discount: {}", early_bird);
println!("coupon used: {}", coupon);
println!("Final ticket price: {}", final_price);
println!("Free entry: {}", free_entry);


}

