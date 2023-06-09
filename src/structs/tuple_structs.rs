struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn master(show: bool) {
    if show {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("Color: {:?} {:?} {:?}", black.0, black.1, black.2);
        println!("Origin: {:?} {:?} {:?}", origin.0, origin.1, origin.2);
    }
}
