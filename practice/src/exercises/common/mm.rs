#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::collections::HashMap;
    use std::mem::{align_of, size_of};

    enum E {
        A(f64),
        B(HashMap<String, String>),
        C(Result<Vec<u8>, String>),
    }

    struct S1 {
        a: u8,
        b: u16,
        c: u8,
    }

    struct S2 {
        a: u8,
        c: u8,
        b: u16,
    }

    macro_rules! show_size {
        (header) => {
            println!(
                "{:<24} {:>4} {} {}",
                "Type", "T", "Option<T>", "Result<T, io::Error>"
            );
            println!("{}", "-".repeat(64));
        };
        ($t:ty) => {
            println!(
                "{:<24} {:4} {:8} {:12}",
                stringify!($t),
                size_of::<$t>(),
                size_of::<Option<$t>>(),
                size_of::<Result<$t, std::io::Error>>(),
            )
        };
    }

    #[test]
    fn main() {
        println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
        println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    }

    #[test]
    fn show_size() {
        show_size!(header);
        show_size!(u8);
        show_size!(f64);
        show_size!(&u8);
        show_size!(Box<u8>);
        show_size!(&[u8]);
        show_size!(String);
        show_size!(Vec<u8>);
        show_size!(HashMap<String, String>);
        show_size!(Result<String,()>);
        show_size!(E);
    }
}
