mod en;
mod flow;
mod mm;
pub mod owner;
mod smart_pointers;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[test]
    fn test_arc_str() {
        let my_string = "hello world";
        let arc_str: Arc<str> = Arc::from(my_string);
        println!("{}", arc_str);
    }
}
