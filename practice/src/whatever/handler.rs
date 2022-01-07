use anyhow::Result;

fn halves_if_even(i: i32) -> Result<i32> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        // Workaround for crates that intentionally contained `{}` in an error message
        // anyhow::bail!("oh no, error");
        Err(anyhow::anyhow!("another error"))
    }
}

fn do_the_thing(i: i32) -> Result<i32> {
    let i = match halves_if_even(i) {
        Ok(i) => i,
        Err(e) => return Err(e)
    };
    Ok(i)
}

fn do_the_same_thing(i: i32) -> Result<i32> {
    // use question mark to exact correct value
    let i = halves_if_even(i)?;
    Ok(i)
}

#[test]
fn test1() {
    let _a = halves_if_even(1).unwrap();
    let _b = do_the_same_thing(1).unwrap_err();
    let _c = do_the_thing(1).unwrap_or(1);
}
