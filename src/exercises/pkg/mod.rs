mod container;
pub mod generic;
pub mod obj;
mod pattern;
mod unsafety;

#[cfg(test)]
mod rsa {
    use std::{fs::File, io::Read};
    use anyhow::Result;
    use base64::Engine;
    use openssl::rsa::{Padding, Rsa};

    #[test]
    pub fn rsa_sample() -> Result<()> {
        let mut f = File::open("secret.pem")?;
        let mut buf: Vec<u8> = Vec::with_capacity(10 * 1024);
        f.read(&mut buf)?;
        let secret = "big secret".to_owned();
        let rsa = Rsa::public_key_from_pem(&buf[..])?;
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let len = rsa.public_encrypt(secret.as_bytes(), &mut buf, Padding::PKCS1)?;

        let encrypted_data = &buf[..len];
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::GeneralPurposeConfig::new(),
        );

        let ss = engine.encode(encrypted_data);
        println!("{}", ss);

        Ok(())
    }
}
