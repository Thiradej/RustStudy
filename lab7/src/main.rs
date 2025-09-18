//1

// fn swap_elements <T> (a: Vec<T>, b: Vec<T>) -> (Vec<T>, Vec<T>) {
//     (b, a)
// }

// fn main() {
//     let vec1 = vec![1, 2, 3];
//     let vec2 = vec![4, 5, 6];

//     let (new_vec1, new_vec2) = swap_elements(vec1, vec2);

//     println!("{:?}, {:?}", new_vec1, new_vec2);
// }



// 3
// how to use
trait SimpleAnalyzable {
    fn mean(&self) -> f64;
    fn filter <F> (&self, predicate: F) -> Self where F:Fn(&f64) -> bool;
}

//Use it
impl SimpleAnalyzable for Vec<f64> {

    fn mean(&self) -> f64{
        let sum: f64 = self.iter().sum();
        sum / self.len() as f64
    }

    fn filter<F>(&self, predicate: F) -> Self
    where 
        F: Fn(&f64) -> bool{
            self.iter().cloned().filter(predicate).collect()
        }
}


struct SimpleDataset {
    data: Vec<f64>
}

impl SimpleAnalyzable for SimpleDataset{
    fn mean(&self) -> f64{
        let sum: f64 = self.data.iter().sum();
        sum / self.data.len() as f64
    }
    fn filter<F>(&self, predicate: F) -> Self
    where 
        F: Fn(&f64) -> bool
        {
            SimpleDataset {
                data: self.data.iter().cloned().filter(predicate).collect(),
            }
        }
}

impl SimpleDataset {
    fn filter <F> (&self, predicate: F) -> Self
    where F:Fn(&f64) -> bool{
        SimpleDataset {
            data: self.data.iter().cloned().filter(predicate).collect(),
        }
    }
    fn new (input: Vec<f64>) -> SimpleDataset{
         return SimpleDataset {data: input};
    }
}


fn main() {
    let data = SimpleDataset::new(vec![1.0, 2.0, 3.0]);
    println!("Mean: {}", data.mean());

    let filter = data.filter(|x| *x > 1.5);
    println!("Filter: {}", filter.mean());
}


//2
struct DataStore <T> {
    item: Vec<T>
}

impl <T> DataStore <T>{
    fn new() -> Self {
        Self { item: Vec::new() }
    }
    fn add_item(&mut self,item: T){
        self.item.push(item)
    }
    fn remove_item(&mut self,index: usize)-> Option<T> {
        if  index < self.item.len() {
            return Some(self.item.remove(index));
        }else {
            return None;
        }
    }
    fn get_item(&mut self,index: usize) -> Option<&T> {
        return self.item.get(index);
    }
    fn find_item <F>(&self, predicate: F) -> Option<&T> where F:Fn(&T) -> bool {
        return self.item.iter().find(|&x| predicate(x));
    }
}

enum DataType <T> {
    Number(T),
    Text(String),
    Boolean(bool),
}

// impl<T: std::fmt::Display> DataType<T> {
//     fn print(&self) {
//         match self {
//             DataType::Number(n) => println!("Number: {}", n),
//             DataType::Text(s) => println!("Text: {}", s),
//             DataType::Boolean(b) => println!("Boolean: {}", b),
//         }
//     }
// }

// fn main() {
    
// }