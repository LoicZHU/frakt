# generate/update rustdoc
echo '+rustdoc src/main.rs'
rustdoc src/main.rs

echo '+rustdoc complex/src/lib.rs --crate-name complex'
rustdoc complex/src/lib.rs --crate-name complex

echo '+rustdoc server/src/lib.rs --crate-name server'
rustdoc server/src/lib.rs --crate-name server

echo '+rustdoc shared/src/lib.rs --crate-name shared'
rustdoc shared/src/lib.rs --crate-name shared

echo '+rustdoc worker/src/lib.rs --crate-name worker'
rustdoc worker/src/lib.rs --crate-name worker