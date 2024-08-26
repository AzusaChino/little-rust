#[cfg(test)]
mod tests {
    use std::{process, rc::Rc};

    const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
    const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
    const CONTEXT: usize = 2;

    fn sum(data: Vec<u32>) -> u32 {
        data.iter().sum()
    }

    fn sum_ref(data: &Vec<u32>) -> u32 {
        // 值的地址会改变么？引用的地址会改变么？
        println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
        data.iter().sum()
    }

    // generate a temporary file name that is hopefully unique
    #[inline]
    fn temp_file() -> String {
        let thread_id: String = format!("{:?}", std::thread::current().id())
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        format!("./temp_{}_{}", process::id(), thread_id)
    }

    #[test]
    fn run_sum() {
        let data = vec![1, 2, 3, 4];
        sum(data.clone());
        let data1 = &data;

        // 值的地址是什么？引用的地址又是什么？
        println!(
            "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
            &data, data1, &&data, &data1
        );
        println!("sum of data1: {}", sum_ref(data1));
        // 堆上数据的地址是什么？
        println!(
            "addr of items: [{:p}, {:p}, {:p}, {:p}]",
            &data[0], &data[1], &data[2], &data[3]
        );
    }

    #[test]
    fn print() {
        println!("{:?} {:?} {:?}", RUSTC_COLOR_ARGS, I_AM_DONE_REGEX, CONTEXT);
        temp_file();
        let _data = include_bytes!("lib.rs");
        let rc = Rc::new(1);
        let _ = rc.clone();
    }
}
