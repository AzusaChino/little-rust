#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        ops::{Deref, Mul},
    };

    use itertools::Itertools;

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
        names.retain(|name| name.starts_with('A'));
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

        for s in "Never:Give:Up".split(':') {
            println!("{}", s);
        }

        // ["", "Hi", "There", ""]
        let s: Vec<_> = "::Hi::There::".split("::").collect();
        println!("{:?}", s);

        // eliminate the empty strings at the end by using split_termitor
        let s: Vec<_> = "Mr.T.".split_terminator('.').collect();
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

        // drop(nums);

        for (i, l) in "abc".chars().enumerate() {
            println!("#{} {}", i + 1, l);
        }

        if let Some(n) = iter.next() {
            println!("name is {}", n);
        }

        let ll = al.nth(3);
        if let Some(ll) = ll {
            println!("fourth is {}", ll);
        }

        // collect iterators into collections
        let _: Vec<_> = (1..10).collect();
        let _nums = (1..10).collect::<Vec<_>>();

        // change which items are being iterated over
        let all_nums = 1..;
        let nums: Vec<_> = all_nums.take(5).collect();
        println!("the first five: {:?}", nums);

        let _: Vec<_> = (0..11).skip(2).collect();

        let _: Vec<_> = (0..).take_while(|x| x * x < 50).collect_vec();

        let countries = ["a", "b", "ab", "ac"];

        let _: Vec<_> = countries.iter().filter(|x| x.contains('a')).collect();

        if let Some(c) = countries.iter().find(|ctry| ctry.starts_with('a')) {
            println!("{}", c);
        }

        if let Some(id) = countries.iter().position(|cry| cry.ends_with('b')) {
            println!("{}", id);
        }

        let _sum: i32 = (1..11).sum();
        let _pdt: i32 = (1..11).product();

        // combine iterators
        let _some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
        let _other: Vec<_> = (1..4).chain(10..14).collect();

        let swiss_postcodes = [8957, 5000, 5034];
        let swiss_towns = ["Spreitenbach", "Aarau", "Suhr"];

        let _zipped: Vec<_> = swiss_postcodes.iter().zip(swiss_towns.iter()).collect();
        println!("{:?}", _zipped);

        // zip is lazy, use two infine ranges
        let _: Vec<_> = (b'A'..)
            .zip(1..)
            .take(10)
            .map(|(ch, num)| (ch as char, num))
            .collect();
    }

    // FIFO queue
    #[test]
    fn vec_deque() {
        let mut orders = VecDeque::new();
        orders.push_back("oysters");
        orders.push_back("fish and chips");

        if let Some(pp) = orders.pop_front() {
            println!("{}", pp);
        }

        let mut some_queue = VecDeque::with_capacity(5usize);
        some_queue.push_back(1);
        some_queue.push_back(2);
        some_queue.push_back(3);
        some_queue.push_back(4);
        some_queue.push_back(5);

        some_queue.swap_remove_back(2);
        println!("{:?}", some_queue);

        some_queue.swap_remove_front(2);
        println!("{:?}", some_queue);
    }

    #[test]
    fn hash_map() {
        let mut mp = HashMap::new();
        mp.insert("1", 1);

        for (k, v) in &mp {
            println!("{}: {}", k, v);
        }

        mp.entry("2").and_modify(|v| *v *= 10).or_insert(2);
    }

    #[test]
    fn hash_set() {
        let one_five: HashSet<_> = (1..=5).collect();
        let five_ten: HashSet<_> = (5..=10).collect();
        let one_ten: HashSet<_> = (1..=10).collect();

        let is_disjoint = one_five.is_disjoint(&five_ten);
        println!("{}", is_disjoint);

        let _ = one_five.is_subset(&one_ten);

        let _dif = one_five.difference(&five_ten);

        let _ = one_five.intersection(&five_ten);
    }

    struct Fibonacci {
        cur: u32,
        next: u32,
    }

    impl Default for Fibonacci {
        fn default() -> Self {
            Self {
                cur: Default::default(),
                next: 1,
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let old = self.cur;
            self.cur = self.next;
            self.next += old;
            Some(old)
        }
    }

    #[test]
    fn fib_itr() {
        let fib: Vec<_> = Fibonacci::default().take(10).collect();
        println!("{:?}", fib);
    }

    struct SquaredVec<T>
    where
        T: Mul + Copy,
    {
        vec: Vec<T::Output>,
    }

    impl<T> SquaredVec<T>
    where
        T: Mul + Copy,
    {
        fn new() -> Self {
            Self { vec: Vec::new() }
        }
        fn push(&mut self, item: T) {
            self.vec.push(item * item);
        }
    }

    impl<T> Deref for SquaredVec<T>
    where
        T: Mul + Copy,
    {
        type Target = [T::Output];

        fn deref(&self) -> &Self::Target {
            &self.vec
        }
    }

    #[test]
    fn sq_itr() {
        let mut sq = SquaredVec::new();
        sq.push(1);
        sq.push(2);
        for (i, n) in sq.iter().enumerate() {
            println!("{},{}", i + 1, n);
        }
    }
}
