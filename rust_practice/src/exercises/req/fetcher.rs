#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn http_md() {
        // 函数本身是一个指向 TEXT 段起始位置的指针。我们可以看到它可以 cast 成 *const () 。
        println!("{:p}", http_md as *const ());
        let url = "https://www.rust-lang.org";
        let output = "output.md";

        println!("fetching url: {}", url);
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let body = reqwest::get(url).await.unwrap().text().await.unwrap();
            let md = html2md::parse_html(&body);
            fs::write(output, md.as_bytes()).unwrap();

            println!("Converted markdown has been saved in {}.", output);
        });
    }
}
