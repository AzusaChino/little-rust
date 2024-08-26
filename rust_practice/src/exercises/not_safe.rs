//! 1. 对裸指针解引用
//! 2. 读写可变静态变量
//! 3. 读 union 或者写 union 的非 Copy 成员
//! 4. 调用 unsafe 函数

#[cfg(test)]
mod tests {

    // raw pointer
    //   *const T
    //   *mut T
    #[test]
    fn test_unsafe() {
        let x = 1_i32;
        let y: u32 = 1;

        // let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64;
        // unsafe {
        //     *raw_mut = -1;
        // }

        println!("{:X}, {:X}", x, y);

        let x = vec![1, 2, 3, 4, 5];

        unsafe {
            let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
            println!("{} {} {}", t.0, t.1, t.2);
        }
    }

    #[test]
    fn test_raw_to_ref() {
        let p: &i32 = raw_to_ref(std::ptr::null::<i32>());
        println!("{}", p);
    }

    fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
        unsafe { &*p }
    }
}
