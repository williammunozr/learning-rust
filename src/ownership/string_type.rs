pub fn master() {
    println!("--- String Type ---");

    let mut s = String::from("hallo");

    s.push_str(", Welt"); // push_str() appends a literal to a String

    println!("{s}");
}
