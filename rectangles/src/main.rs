#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let _width1 = 30;
    let _height1 = 50;
    let _rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of rectangle is {} square pixels.",
        area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn _area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _area(width: u32, height: u32) -> u32 {
    width * height
}