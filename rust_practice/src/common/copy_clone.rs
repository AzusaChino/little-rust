#![allow(unused)]

use std::{
    collections::LinkedList,
    ops::{Add, Deref, DerefMut, Index},
};

#[derive(Debug, Clone, Copy)]
pub struct PointCloneAndCopy {
    pub x: u64,
}

#[derive(Debug, Clone)]
pub struct PointCloneOnly {
    pub x: u64,
}

fn test_copy_and_clone() {
    let p1 = PointCloneAndCopy { x: 0 };
    let p2 = p1; // `Copy`, gets copied automatically
    println!("{:?} {:?}", p1, p2);
}

fn test_clone_only() {
    let p1 = PointCloneOnly { x: 0 };
    let p2 = p1; // no `Copy` trait, move
    println!("{:?}", p2)
}

fn json_() {
    let code = 200;
    let features = vec!["serde", "json"];

    let _val = serde_json::json!({
        "code": code,
        "status": code == 200,
        "payload": {
            features[0]: features[1]
        }
    });
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl Add for Complex {
    type Output = Self;

    // 移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    // 借用
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    // 借用
    fn add(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}

struct List<T>(LinkedList<T>);

impl<T> Deref for List<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.iter().nth(index).unwrap()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn foo() {
        let mut data = vec!['a', 'b', 'c', 'd'];
        // Non Lexical Lifetime
        let slice = &mut data[..];
        capitalize(slice);

        data.push('e');

        println!("{:?}", data);
    }

    fn capitalize(data: &mut [char]) {
        for c in data {
            c.make_ascii_uppercase()
        }
    }
}
