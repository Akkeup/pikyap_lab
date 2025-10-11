#![allow(unused)]

trait GeometricalShape { // сюда закинуть print_info()
    fn find_area(&self) -> f64;
}

// rectangle --point
struct Rectangle {
    name: String,
    width: f64,
    height: f64,
}
impl GeometricalShape for Rectangle {
    fn find_area(&self) -> f64 {
        self.width * self.height
    }
}
impl Rectangle {
    fn print_info(&self) {
        println!("Фигура: {}, Ширина: {}, Длина: {}, Площадь: {}",
            self.name,
            self.height,
            self.width,
            self.find_area()
        );
    }
}
// rectangle --endpoint

// Square --point
struct Square {
    name: String,
    side: f64,
}
impl GeometricalShape for Square {
    fn find_area(&self) -> f64 {
        self.side*self.side
    }
}
impl Square {
    fn print_info(&self) {
        println!("Фигура: {}, Сторона: {}, Площадь {}",
            self.name,
            self.side,
            self.find_area()
        );
    }
}
// Square --endpoint

// circle --point
static PI: f64 = 3.141592;
struct Circle {
    name: String,
    radius: f64,
}
impl GeometricalShape for Circle {
    fn find_area(&self) -> f64 {
        PI*self.radius*self.radius
    }
}
impl Circle {
    fn print_info(&self) {
        println!("Фигура: {}, Радиус: {}, Площадь: {}", 
            self.name, 
            self.radius, 
            self.find_area()
        );
    }   
}
// circle --endpoint

enum Shape {
    Rectangle(Rectangle),
    Square(Square),
    Circle(Circle),
}
impl Shape {
    fn print_info(&self) {
        match self {
            Shape::Rectangle(r) => r.print_info(),
            Shape::Square(s) => s.print_info(),
            Shape::Circle(c) => c.print_info(),
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec! [
        Shape::Rectangle(Rectangle { name: "Прямоугольник".to_string(), width: 12.0, height: 3.9 }),
        Shape::Square(Square {name: "Квадрат".to_string(), side: 11.8}),
        Shape::Circle(Circle { name: "Круг".to_string(), radius: 4.3 })
    ];

    for shape in shapes {
        shape.print_info();
        println!("-=-=-=-=-=-=-");
    }
}