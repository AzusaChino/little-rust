fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["proto/hello.proto"], &["src/protos/"])?;
    Ok(())
}
