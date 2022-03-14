#![allow(unused)]

// 使用unsafe把裸指针转换成了共享引用
fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
    unsafe { &*p }
}

fn raw_to_ref_ok<'a>(p: *const i32) -> Option<&'a i32> {
    if p.is_null() {
        None
    } else {
        unsafe { Some(&*p) }
    }
}

#[cfg(test)]
mod test {
    use super::raw_to_ref;
    #[test]
    fn main() {
        let p: &i32 = raw_to_ref(std::ptr::null::<i32>());
        println!("{}", p);
    }

    #[test]
    fn main_() {
        let p: Option<&i32> = super::raw_to_ref_ok(std::ptr::null::<i32>());
        println!("{:?}", p);
    }
}
