fn main() -> std::io::Result<()> {
    prost_build::Config::new()
    .out_dir("src/pb")
    .compile_protos(&["src/proto/abi.proto", "src/proto/hello.proto"], &["src"])?;
    // prost_build::Config::new()
    //     .out_dir("src/excercises/pb")
    //     .compile_protos(&["src/proto/abi.proto", "src/proto/hello.proto"], &["src"])?;

    Ok(())
}
