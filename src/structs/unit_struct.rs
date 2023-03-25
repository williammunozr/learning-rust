struct AlwaysEqual;

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn master() {
    let _subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 70,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
