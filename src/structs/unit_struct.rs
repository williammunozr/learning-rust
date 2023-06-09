struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn master(show: bool) {
    if show {
        let _subject = AlwaysEqual;

        let rect1 = Rectangle {
            width: 30,
            height: 70,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        // to work, we need to annotate the struct Rectangle with:
        // #[derive(Debug)]
        println!("rect1 is {:?}", rect1);
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
