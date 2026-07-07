// pub struct BankAccount {
//     owner: String,
//     balance: u64,
// }

// impl BankAccount {
//     pub fn new(owner: String) -> BankAccount {
//         BankAccount {owner, balance: 0}
//     }

//     pub fn deposit(&mut self, amount: u64) {
//         self.balance += amount
//     }

//     pub fn withdraw(&mut self, amount: u64) -> Result<(), String> {
//         if amount > self.balance {
//             Err("No money naja".to_string())
//         }
//         else {
//             self.balance -= amount;
//             Ok(())
//         }
//     }
    
//     pub fn balance(&self) -> u64 {
//         self.balance
//     }

//     pub fn owner(&self) -> &str {
//         &self.owner
//     }
//     }


// fn main() {
    
//     let mut acc = BankAccount::new("Beam".to_string());
//     acc.deposit(1000);
//     BankAccount::deposit(&mut acc, 1000);
//     match acc.withdraw(200) {
//         Ok(()) => println!("withdraw succesful"),
//         Err(e) => println!("{}", e)
//     }
//     println!("owner: {}  balance: {}", acc.owner, acc.balance);
// }



// //2
// fn handle(c: char) -> &'static str {
//     const QUIT: char = 'q';
//     match c {
//         QUIT => {"quit"},
//         'a' | 's' | 'w' | 'd' => {"move"},
//         '0'..='9' => {"digit"}
//         ch if ch.is_ascii_lowercase() => {"lowercase"}
//         _ => {"other"}
//     }

// }
// fn main() {
//     let arr = ['q', 'a', '7', 'x', '%', '9', 'A', 'd'];
//     for c in arr {
//         println!("{} -> {}", c, handle(c))
//     }
// }


// //3
// fn point(p: (i32, i32)) -> &'static str {
//     match p {
//         (x, y) if x > 0 && y > 0 => "I",
//         (x, y) if x < 0 && y > 0 => "II",
//         (x, y) if x < 0 && y < 0 => "III",
//         (x, y) if x > 0 && y < 0 => "IV",
//         (x, y) if x == 0 || y == 0 => "axis",
//         (_, _) => "Eiei"
//     }
// }

// fn main() {
//     let a = vec![(1, 2), (-1, 1), (-2, -2), (3, -2), (0, 0)];
//     for p in a {
//         println!("{:?} => {}", p, point(p))
//     }
// }



pub fn first_hex_digit(maybe: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe else {return Err("None".to_string())};
    let Some(first) = s.chars().next() else {return Err("empty".to_string())};
    let Some(digit) = first.to_digit(16) else {return Err("not hex".to_string())};
    Ok(digit)
}
pub fn pop_all(s: &mut String) -> Vec<char> {
    let mut i = Vec::new();
    while let Some(ch) = String::pop(s) {
        i.push(ch)
    }
    i
}
pub fn print_parse_u8 (s: &str) {
    if let Some(x) = try_parse(s) {
        println!("{}", x)
    }
}
fn try_parse(s: &str) -> Option<u8> {
        match s.parse::<u8>() {
            Ok(char) => Some(char),
            Err(_) => None
        }
}
fn main() {
    println!("{:?}", first_hex_digit(Some("Beef".to_string())));
    println!("{:?}", first_hex_digit(Some("".to_string())));
    println!("{:?}", first_hex_digit(Some("x".to_string())));
    println!("{:?}", first_hex_digit(None));

    let mut k = String::from("abc123");
    println!("{:?}", pop_all(&mut k));
    print_parse_u8("42");
    print_parse_u8("x");
}