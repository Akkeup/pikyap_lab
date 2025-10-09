#![allow(unused)]

trait GeometricShape { // геометрическая фигура 
    fn print_info(&self);
    fn find_area(&self) -> f64; // нахождение площади фигуры
}

// structs --point
struct Rectangle { // прямугольник
    name: String,
    width: f64,
    height: f64,
}

struct Square { // квадрат
    name: String,
    side: f64,
}

static PI: f64 = 3.141592;
struct Circle { // круг
    name: String,
    radius: f64,
}
// structs --endpoint

// impl --point
impl GeometricShape for Rectangle { 
    fn find_area(&self) -> f64 {
        self.width * self.height
    }

    fn print_info(&self) {
        println!("Name: {}, Width: {}, Height: {}", self.name, self.width, self.height);
    }
}
// impl Rectangle {
//     fn print_info(&self) {
//         println!("Name: {}, Width: {}, Height: {}", self.name, self.width, self.height);
//     }
// }

impl GeometricShape for Square {
    fn find_area(&self) -> f64 {
        self.side * self.side
    }

    fn print_info(&self) {
        println!("Name: {}, Side: {}", self.name, self.side);
    }
}
// impl Square {
//     fn print_info(&self) {
//         println!("Name: {}, Side: {}", self.name, self.side);
//     }
// }

impl GeometricShape for Circle { 
    fn find_area(&self) -> f64 {
        PI*self.radius*self.radius
    }

    fn print_info(&self) {
        println!("Name: {}, Radius: {}", self.name, self.radius);
    }
}
// impl Circle {
//     fn print_info(&self) {
//         println!("Name: {}, Radius: {}", self.name, self.radius);
//     }
// }

// fn f_find_area<T: GeometricShape>(item: &T) {
//     item.find_area();
// }
// impl --endpoint

fn main() {
    let shapes: Vec<Box<dyn GeometricShape>> = vec! [
        Box::new(Rectangle { name: "Rectangle".to_string(), width: 11.0, height: 12.0 }),
        Box::new(Square { name: "Square".to_string(), side: 64.0 }),
        Box::new(Circle { name: "Circle".to_string(), radius: 5.0 }),
    ];

    for shape in shapes {
        shape.print_info();
        println!("Area: {}", shape.find_area());
        println!("-=-=-=-=-=-=-");
    }
}
