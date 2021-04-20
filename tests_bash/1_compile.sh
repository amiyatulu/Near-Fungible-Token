set -e
cd ..
cd w-near-141
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/w_near.wasm ./res/