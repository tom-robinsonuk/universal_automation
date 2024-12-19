fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the protoc binary provided by protoc-bin-vendored
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    println!("Using protoc binary at: {:?}", protoc_bin_vendored::protoc_bin_path()?);

    // Define where the generated files should go
    let out_dir = "src/generated";
    std::fs::create_dir_all(out_dir)?; // Create the directory if it doesn't exist

    // Compile the proto file
    tonic_build::configure()
        .out_dir(out_dir) // Output directory for generated code
        .compile_protos(&["../proto/automation.proto"], &["../proto"])?;

    println!("Proto file compiled successfully to {:?}", out_dir);
    Ok(())
}
