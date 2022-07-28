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

    for _abc in 1..2 {}
}