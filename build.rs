fn main() {
    tonic_build::configure()
        .out_dir("./src")
        .compile(&["acme.proto"], &["./proto"])
        .unwrap_or_else(|e| panic!("Failed to compile proto(s) {:?}", e));
}
