#[cfg(test)]
mod tests {

    #[test]
    fn vec() {
        let _ = vec!["a", "b", "c"];

        let mut fruits = Vec::new();
        fruits.push("apple");
        fruits.push("banana");

        let last = fruits.pop();

        if let Some(last) = last {
            println!("removed {} from fruits {:?}", last, fruits);
        }

        fruits.insert(1usize, "grape");

        // swap elements
        fruits.swap(0, 1);

        // access elements
        if let Some(first) = fruits.first() {
            println!("{}", first);
        }

        if let Some(last) = fruits.last() {
            println!("{}", last);
        }

        if let Some(second) = fruits.get(1) {
            println!("second fruit: {}", second);
        }

        // access arbitrary elements without boundary checking
        let 第二个 = fruits[1];
        println!("第二个是 {}", 第二个);

        // initialize the vector with a value
        let zeros = vec![0; 5];
        println!("{:?}", zeros);

        let mut nums = vec![1, 2, 4, 5, 6];
        let second = nums.remove(1);
        println!("removed {} from {:?}", second, nums);

        // Filter in place
        let mut names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
        names.retain(|name| name.starts_with("A"));
        println!("{:?}", names);
        println!("{}", names.contains(&"Alex"));

        // remove consecutive duplicates
        let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
        nums.dedup();
        println!("deduped: {:?}", nums);

        // unsorted - undefined behaviour
        let mut nums = vec![2, 1, 4, 2, 3, 5, 1, 2];
        nums.dedup();
        println!("deduped, unsorted: {:?}", nums);

        nums.sort();
        nums.reverse();

        // consume iterator over a range
        let mut alphabet = vec!['a', 'b', 'c'];
        print!("the first two letters of the alphabet are: ");
        for l in alphabet.drain(..2) {
            print!(" {}", l);
        }

        let mut a = nums.split_off(2usize);
        nums.append(&mut a);
    }

    #[test]
    fn string() {
        let mut s = String::new();
        s.push('h');
        s.push('e');
        println!("{}", s);

        // always be valid UTF-8
        let s = "中文 😊 🍺 *^____^*".to_string();
        println!("{}", s);

        let mut s = String::from("Hello, ");
        s.push_str("World!");

        for ch in s.chars() {
            print!("{}", ch);
        }

        let (first, second) = "HelloThere".split_at(5);
        println!("{}, {}", first, second);

        let haiku = "\
        she watches \n\
        satisfied after love\n\
        he lies\n\
        looking up at nothing\n\
        ";
        for line in haiku.lines() {
            println!("\t {}.", line);
        }

        for s in "Never:Give:Up".split(":") {
            println!("{}", s);
        }

        // ["", "Hi", "There", ""]
        let s: Vec<_> = "::Hi::There::".split("::").collect();
        println!("{:?}", s);

        // eliminate the empty strings at the end by using split_termitor
        let s: Vec<_> = "Mr.T.".split_terminator(".").collect();
        println!("{:?}", s);

        // split at numeric
        for s in "I'm2fast4you".split(char::is_numeric) {
            println!("{}", s);
        }

        // split only a certain amount oftimes
        for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
            println!("{}", s);
        }

        // Get only the substrings that match a pattern
        for c in "The Dark Knight rises".matches(char::is_uppercase) {
            println!("{}", c);
        }

        let saying = "The early bird gets the worm";
        let _ = saying.starts_with("The");
        let _ = saying.ends_with("worm");
        let _ = saying.contains("bird");

        // remove whitespace

        let spaces_a_lot = "     I love     spaces      ";
        let s: Vec<_> = spaces_a_lot.split(' ').collect();
        println!("{:?}", s);
        println!("{:?}", spaces_a_lot.split_whitespace().collect::<Vec<_>>());

        let num = "12".parse::<i32>();
        if let Ok(num) = num {
            println!("{}", num * num);
        }

        let s = "My dad is the best dad";
        println!("{}", s.replace("dad", "mom"));
        println!("{}", s.to_uppercase());
        println!("{}", s.repeat(3));
    }

    #[test]
    fn itr() {
        let names = vec!["joe", "miranda", "alice"];

        let mut iter = names.iter();
        let mut al = "abcdefg".chars();
        let _ = 0..=10;
        let nums = 0..;

        drop(nums);

        for (i, l) in "abc".chars().enumerate() {
            println!("#{} {}", i + 1, l);
        }

        if let Some(n) = iter.next() {
            println!("name is {}", n);
        }

        let ll = al.nth(3);
        match ll {
            Some(ll) => println!("fourth is {}", ll),
            None => {}
        }
    }
}
