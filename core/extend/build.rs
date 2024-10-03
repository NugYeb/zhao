






fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/call_core.proto")?;
    tonic_build::compile_protos("../../proto/call_extend.proto")?;

    Ok(())
}
