#[cfg(test)]
mod tests {
    use std::{
        self,
        collections::HashMap,
        net::Ipv6Addr,
        sync::{atomic::AtomicUsize, Arc, RwLock},
    };

    // rayon implements `ParallelIterator`
    use rayon::prelude::*;

    #[test]
    fn test() {
        let legend = "did you ever heard the tragedy of darth plaguels the wise?";
        let words: Vec<_> = legend.split_whitespace().collect();

        words.par_iter().for_each(|v| println!("{v}"));

        let words_with_a: Vec<_> = words.par_iter().filter(|v| v.find('a').is_some()).collect();
        println!("{:?}", words_with_a);
    }

    #[derive(Debug)]
    struct Rectangle {
        height: u32,
        width: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }

        fn perimeter(&self) -> u32 {
            2 * (self.height + self.width)
        }
    }

    fn fibonacci(n: u32) -> u32 {
        if n == 0 || n == 1 {
            n
        } else {
            let (a, b) = rayon::join(|| fibonacci(n - 1), || fibonacci(n - 2));
            a + b
        }
    }

    #[test]
    fn rect() {
        let rect = Rectangle {
            height: 10,
            width: 9,
        };
        // rayon::join makes closures run potentialy in parallel and returns their valus in a tuple
        let (a, p) = rayon::join(|| rect.area(), || rect.perimeter());
        println!("{a}, {p}");

        println!("{}", fibonacci(6));
    }

    #[test]
    fn arc() {
        let some_resources = Arc::new("hello_world".to_owned());
        let a = {
            let sr = Arc::clone(&some_resources);

            std::thread::spawn(move || {
                println!("a said {sr}");
            })
        };
        let b = {
            let sr = Arc::clone(&some_resources);

            std::thread::spawn(move || {
                println!("b said {sr}");
            })
        };

        a.join().expect("fail to exec");
        b.join().expect("fail to exec");
    }

    #[test]
    fn rw() {
        let r = Arc::new(RwLock::new("hello world".to_owned()));
        let t_a = {
            let res = Arc::clone(&r);
            std::thread::spawn(move || {
                for _ in 0..=39 {
                    let res = res.read().expect("fail to attain read lock");
                    println!("reader a said: {res}");
                }
            })
        };

        let t_w = {
            let res = Arc::clone(&r);
            std::thread::spawn(move || {
                for _ in 0..=9 {
                    let mut res = res.write().expect("fail to attain read lock");
                    res.push('!');
                }
            })
        };

        t_a.join().expect("fail to exec");
        t_w.join().expect("fail to exec");
    }

    struct Client {
        ip: Ipv6Addr,
    }

    struct ConnectionHandler {
        clients: RwLock<HashMap<usize, Client>>,
        next_id: AtomicUsize,
    }

    impl Client {
        fn new(ip: Ipv6Addr) -> Self {
            Self { ip }
        }

        fn print(&self) {
            println!("{}", &self.ip)
        }
    }

    impl ConnectionHandler {
        fn new() -> Self {
            Self {
                clients: RwLock::new(HashMap::new()),
                next_id: AtomicUsize::new(0),
            }
        }
        fn client_count(&self) -> usize {
            self.clients.read().expect("fail to lock for reading").len()
        }

        fn add_connection(&self, ip: Ipv6Addr) -> usize {
            let last = self
                .next_id
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.clients
                .write()
                .expect("fail to get lock for writing")
                .insert(last, Client::new(ip));
            last
        }

        fn remove_connection(&self, id: usize) -> Option<()> {
            self.clients
                .write()
                .expect("fail to get lock for writing")
                .remove(&id)
                .and(Some(()))
        }
    }

    #[test]
    fn full() {
        let c = Client::new(Ipv6Addr::LOCALHOST);
        c.print();
        
        let connections = Arc::new(ConnectionHandler::new());
        let connector = {
            let cs = Arc::clone(&connections);
            let dummy_ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
            let ten_millis = std::time::Duration::from_millis(10);
            std::thread::spawn(move || {
                for _ in 0..20 {
                    cs.add_connection(dummy_ip);
                    std::thread::sleep(ten_millis);
                }
            })
        };
        let dis = {
            let cs = Arc::clone(&connections);
            let fifty_millis = std::time::Duration::from_millis(50);
            std::thread::spawn(move || {
                for _ in 0..40 {
                    std::thread::sleep(fifty_millis);
                    cs.remove_connection(2);
                }
            })
        };

        let five_millis = std::time::Duration::from_millis(5);

        for _ in 0..40 {
            let c = connections.client_count();
            println!("active connections: {c}");
            std::thread::sleep(five_millis);
        }

        connector.join().expect("fail to exec");
        dis.join().expect("fail to exec");
    }
}
