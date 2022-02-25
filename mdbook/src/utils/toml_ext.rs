use toml::value::{Table, Value};

pub(crate) trait TomlExt {
    fn read(&self, key: &str) -> Option<&Value>;
    fn read_mut(&mut self, key: &str) -> Option<&mut Value>;

    fn insert(&mut self, key: &str, value: Value);
    fn delete(&mut self, key: &str) -> Option<Value>;
}

// add extensibility for toml::Value
impl TomlExt for Value {
    fn read(&self, key: &str) -> Option<&Value> {
        if let Some((head, tail)) = split(key) {
            self.get(head)?.read(tail)
        } else {
            self.get(key)
        }
    }

    fn read_mut(&mut self, key: &str) -> Option<&mut Value> {
        if let Some((h, t)) = split(key) {
            self.get_mut(h)?.read_mut(t)
        } else {
            self.get_mut(key)
        }
    }

    fn insert(&mut self, key: &str, value: Value) {
        if !self.is_table() {
            *self = Value::Table(Table::new());
        }
        let table = self.as_table_mut().expect("unreachable");

        if let Some((head, tail)) = split(key) {
            table
                .entry(head)
                .or_insert_with(|| Value::Table(Table::new()))
                .insert(tail, value);
        } else {
            table.insert(key.to_string(), value);
        }
    }

    // do not support delete the bigger table key
    fn delete(&mut self, key: &str) -> Option<Value> {
        if let Some((head, tail)) = split(key) {
            self.get_mut(head)?.delete(tail)
        } else if let Some(table) = self.as_table_mut() {
            table.remove(key)
        } else {
            None
        }
    }
}

fn split(key: &str) -> Option<(&str, &str)> {
    let dot = key.find('.')?;
    let (head, tail) = key.split_at(dot);

    // splitting will leave the ".", so remove it
    let tail = &tail[1..];
    Some((head, tail))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // inherit parent using
    use super::*;

    #[test]
    fn read_simple_table() {
        let src = "[table]";
        let value = Value::from_str(src).unwrap();
        let got = value.read("table").unwrap();
        assert!(got.is_table());
    }

    #[test]
    fn read_nested_item() {
        let src = "[table]\nnested=true";
        let value = Value::from_str(src).unwrap();
        let got = value.read("table.nested").unwrap();

        assert_eq!(got, &Value::Boolean(true));
    }

    #[test]
    fn insert_item_at_top_level() {
        let mut value = Value::Table(Table::new());
        let item = Value::Boolean(true);

        // we would use item for comparison later, so call clone
        value.insert("first", item.clone());

        assert_eq!(value.get("first").unwrap(), &item);
    }

    #[test]
    fn delete_item_at_top_level() {
        let src = "top = true";
        let mut value = Value::from_str(src).unwrap();
        let got = value.delete("top").unwrap();

        assert_eq!(got, Value::Boolean(true));
    }

    #[test]
    fn delete_item_at_nested_level() {
        let src = "[table]\n nested = true";
        let mut value = Value::from_str(src).unwrap();

        let got = value.delete("table.nested").unwrap();

        assert_eq!(got, Value::Boolean(true));
    }
}
