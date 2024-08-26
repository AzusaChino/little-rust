#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    fn fib1(n: u8) {
        let (mut a, mut b, mut i) = (1, 1, 2);
        loop {
            let c = a + b;
            a = b;
            b = c;
            i += 1;
            println!("next val is {}", b);
            if i >= n {
                break;
            }
        }
    }

    fn fib2(n: u8) {
        let (mut a, mut b, mut i) = (1, 1, 2);
        while i < n {
            let c = a + b;
            a = b;
            b = c;
            i += 1;
            println!("next val is {}", b);
        }
    }

    fn fib3(n: u8) {
        let (mut a, mut b) = (1, 1);
        for _i in 2..n {
            let c = a + b;
            a = b;
            b = c;
            println!("next val is {}", b);
        }
    }

    #[test]
    fn sample() {
        let mut num = 3;
        if num != 0 {
            println!("ok")
        } else {
            println!("ok ko")
        }

        loop {
            if num > 10 {
                break;
            }
            println!("current: {}", num);
            num += 1;
        }

        let mut count = 0;
        // label with a single quote
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {}", count);

        let mut counter = 1;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);

        for n in 1..2 {
            println!("{}", n)
        }
        fib1(10);
        fib2(10);
        fib3(10);
    }

    #[test]
    fn panic() {
        use std::thread;

        const COUNT: u32 = 100000;

        let global = Arc::new(Mutex::new(0));

        let clone1 = global.clone();
        let th1 = thread::spawn(move || {
            for _ in 0..COUNT {
                match clone1.lock() {
                    Ok(mut value) => *value += 1,
                    Err(poisoned) => {
                        let mut value = poisoned.into_inner();
                        *value += 1;
                    }
                }
            }
        });

        let clone2 = global.clone();
        let th2 = thread::spawn(move || {
            for _ in 0..COUNT {
                let mut value = clone2.lock().unwrap();
                *value -= 1;
                if *value < 100000 {
                    println!("make a panic");
                    panic!("oops")
                }
            }
        });

        th1.join().ok();
        th2.join().ok();
        println!("final value: {:?}", global);
    }
}
