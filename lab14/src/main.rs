use std::f64::consts::PI;

trait ShapeProperties {
    fn area(&self) -> f64 ;
    fn perimeter(&self) -> f64 ;
}
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
#[derive(Debug)]
struct Circle {
    radius: f64
}
impl ShapeProperties for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
impl ShapeProperties for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn print_details<T: ShapeProperties + std::fmt::Debug>(shape: &T){
    println!("Shape: {:?}", shape);
    println!("Area: {:?}", shape.area());
    println!("Perimeter: {:?}", shape.perimeter())
}

fn main() {
    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circle = Circle { radius: 7.5 };
    println!("---Rectangle---");
    print_details(&rect);
    println!("\n--- Circle ---");
    print_details(&circle);
}



// use std::ops::Add;
// use std::fmt;

// #[derive(Debug, Copy, Clone)]
// struct Vector2D {
//     x: f32,
//     y: f32,
// }

// impl Add for Vector2D {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self {
//         Self{x: self.x + rhs.x, y: self.y + rhs.y}
//     }
// }
// impl fmt::Display for Vector2D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }
// fn main() {
//     let v1 = Vector2D { x: 5.0, y: 2.0};
//     let v2 = Vector2D { x: -1.0, y: 3.0};
//     let v3 = v1 + v2;
//     println!("Vector 1: {}", v1);
//     println!("Vector 2: {}", v2);
//     println!("v1 + v2 = {}", v3);
//     println!("Debug format: {:?}", v3);
// }

// use std::fmt::format;

// trait Renderable {
//     fn render(&self) -> String;
// }
// struct Button {
//     label: String
// }
// struct Label{
//     text: String
// }
// struct Container{
//     name: String,
//     children: Vec<Box<dyn Renderable>>
// }
// impl Renderable for Button{
//     fn render(&self) -> String {
//         return format!("Button: [{}]", self.label.clone());
//     }
// }
// impl Renderable for Label {
//     fn render(&self) -> String {
//         return format!("Label: {}", self.text.clone());
//     }
// }
// impl Renderable for Container {
//     fn render(&self) -> String {
//         let mut result = format!("Container ('{}') {{\n", self.name);
//         for child in &self.children{
//             let render = child.render();
//             result.push_str(&format!("  {}\n", child.render()));
//         }result.push('}');
//         result
//     }
// }
// fn main() {
//     let mut inner_container = Container {
//         name: "Login Form".to_string(),
//         children: Vec::new(),
//     };
//     inner_container.children.push(Box::new(Label {text: "Username".to_string()}));
//     inner_container.children.push(Box::new(Button {label: "Submit".to_string()}));
//     let mut screen: Vec<Box< dyn Renderable >> = Vec::new();
//     screen.push(Box::new(Label { text: "Welcome to my App!".to_string()}));
//     screen.push(Box::new(inner_container));
//     screen.push(Box::new(Button{label: "Sign Out".to_string()}));
//     // render everythings on the screen
//     println!("--- Rendering Screen ---");
//     for component in screen {
//         println!("{}", component.render())
//     }
// }