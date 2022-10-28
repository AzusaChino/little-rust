#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn http_md() {
        let url = "https://www.rust-lang.org";
        let output = "output.md";

        println!("fetching url: {}", url);

        let body = reqwest::blocking::get(url).unwrap().text().unwrap();
        let md = html2md::parse_html(&body);
        fs::write(output, md.as_bytes()).unwrap();

        println!("Converted markdown has been saved in {}.", output);
    }
}
