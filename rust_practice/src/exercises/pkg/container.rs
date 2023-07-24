#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        mem,
        rc::{Rc, Weak},
    };

    fn extend_vec(v: &mut Vec<i32>) {
        // Vec<T> 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
        // capacity 会变成 64
        (2..34).for_each(|i| v.push(i));
    }

    fn print_vec<T>(name: &str, data: Vec<T>) {
        let p: [usize; 3] = unsafe { mem::transmute(data) };
        // 打印 Vec<T> 的堆地址，capacity，len
        println!("{}: 0x{:x}, {}, {}", name, p[0], p[1], p[2]);
    }

    #[test]
    fn main() {
        let mut v = vec![1];
        let v1: Vec<i32> = Vec::with_capacity(8);

        print_vec("v1", v1);

        // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
        println!("heap start: {:p}", &v[0] as *const i32);
        extend_vec(&mut v);

        // heap 地址改变了！这就是为什么可变引用和不可变引用不能共存的原因
        println!("new heap start: {:p}", &v[0] as *const i32);
        print_vec("v", v);
    }

    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    #[test]
    fn ref_check() {
        let owner: Rc<Owner> = Rc::new(Owner {
            name: "chris".to_owned(),
            gadgets: RefCell::new(Vec::new()),
        });

        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: owner.clone(),
        });
        let gadget2 = Rc::new(Gadget {
            id: 2,
            owner: owner.clone(),
        });

        // update gadgets for owner, use Weak to prevent loop-reference
        owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
        owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

        for g_opt in owner.gadgets.borrow().iter() {
            // this will panic, if Weak already get droped
            let g = g_opt.upgrade().unwrap();

            println!("gadget {} owned by {}", g.id, g.owner.name);
        }
    }
}
