enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}
fn main() {
   let shapes = vec![
       Shape::Circle(5.0),
       Shape::Rectangle(5.0, 2.0),
       Shape::Square(5.0),
   ];

   let total_area: f64 = shapes.iter().map(|shape| {
       match shape {
           Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
           Shape::Rectangle(width, height) => width * height,
           Shape::Square(side) => side * side,
       }
   }).sum();

    println!("Total area: {} sq. units", total_area);
}
