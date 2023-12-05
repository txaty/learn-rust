fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        areaRectangle(&rect1)
    );
}

fn areaOrigin(width: u32, height: u32) -> u32 {
    width * height
}

fn areaTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn areaRectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
