enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]

enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let disk_type = DiskType::SSD;
    // Can't compare them like this!
    // if disk_type == DiskType::SSD {
    //     println!("It's an SSD!");
    // } else {
    //     println!("It's not an SSD!");
    // }

    // Instead, we can use a match statement
    match disk_type {
        DiskType::SSD => println!("It's an SSD!"),
        DiskType::HDD => println!("It's not an SSD!"),
    }

    let disk_size = DiskSize::GB(512);
    println!("Disk size: {:?}", disk_size);

    let shape = Shape::Rectangle(3.0, 5.8);
    match shape {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
        Shape::Square(side) => println!("Square with side: {}", side),
    }

}