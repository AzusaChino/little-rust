// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

mod super_power {
    fn a() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_a() {
            a();
        }
    }
}