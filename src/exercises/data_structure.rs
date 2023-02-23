#[cfg(test)]
mod tests {
    extern crate lazy_static;
    extern crate regex;

    // created only the first time it is used
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{collections::HashMap, sync::RwLock};

    // global static immutable
    lazy_static! {
        static ref CURRENCIES: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("EUR", "Euro");
            m.insert("USD", "U.S. Dollar");
            m.insert("CNY", "RMB");
            m
        };
    }

    // global static mutable
    lazy_static! {
        static ref CLIENTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
    }

    // local static
    fn extract_day(date: &str) -> Option<&str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("fail to create regex");
        }

        RE.captures(date)
            .and_then(|cap| cap.get(1).map(|day| day.as_str()))
    }

    #[test]
    fn lazy() {
        let usd = CURRENCIES.get("USD");
        if let Some(usd) = usd {
            println!("{}", usd);
        }

        // mutate the global
        CLIENTS
            .write()
            .expect("fail to unlock clients for writing")
            .push("192.168.0.1".to_owned());

        let clients = CLIENTS.read().expect("fail to get read lock");
        println!("{}", clients.get(0).expect("fail to get first"));

        let date = "12.01.2018";
        if let Some(day) = extract_day(date) {
            println!("{}", day);
        }
    }
}
