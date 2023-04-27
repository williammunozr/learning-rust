pub mod generics;
pub mod closures;
pub mod iterators;
pub mod doc;
pub mod tests;
pub mod boxes;
pub mod custom_smart_pointer;
pub mod rc_t;
pub mod messenger;
pub mod tree_data_structure;
pub mod threads;
pub mod joinhandle;
pub mod message_passing;

pub fn print_title(title: &str) {
    println!(" ");
    println!("### {title} ###");
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more form {}...",
            self.summarize_author()
        )
    }
}
