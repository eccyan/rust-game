fn main() {
    tonic_build::compile_protos("proto/game.proto").unwrap();
    tonic_build::compile_protos("proto/world.proto").unwrap();
}
