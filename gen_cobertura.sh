export RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly build
export LLVM_PROFILE_FILE="cov-%p-%m.profraw"
rm *.profraw || true
cargo +nightly test
grcov . -s . --binary-path ./target/debug/ -t cobertura --branch --ignore-not-existing --ignore "*cargo*" -o ./cobertura.xml