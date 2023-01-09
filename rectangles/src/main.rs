fn main() {
    let _width1 = 30;
    let _height1 = 50;
    let rect1 = (30, 50);

    println!(
        "The area of rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _area(width: u32, height: u32) -> u32 {
    width * height
}