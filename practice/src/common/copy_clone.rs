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

#[test]
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

#[allow(unused)]
fn unused() {
    test_copy_and_clone();
    test_clone_only();
}
