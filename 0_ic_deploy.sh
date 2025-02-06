# !bin/sh
cargo build --release --target wasm32-unknown-unknown --manifest-path src/client/Cargo.toml
cargo build --release --target wasm32-unknown-unknown --manifest-path src/server/Cargo.toml
cargo build --release --target wasm32-unknown-unknown --manifest-path src/proxy/Cargo.toml

candid-extractor target/wasm32-unknown-unknown/release/client.wasm > src/client/client.did
candid-extractor target/wasm32-unknown-unknown/release/server.wasm > src/server/server.did
candid-extractor target/wasm32-unknown-unknown/release/proxy.wasm > src/proxy/proxy.did
dfx deploy -y --ic

CANISTER_ID=$(yq e '.server_same_subnet.ic' canister_ids.json)

CLIENT_COUNT=10

while [ $CLIENT_COUNT -gt 0 ]; do
    if [ $CLIENT_COUNT -eq 10 ]; then
        CLIENT=client
    else
        CLIENT=client$(($CLIENT_COUNT+1))
    fi
    dfx canister call $CLIENT set_server $CANISTER_ID --ic
    CLIENT_COUNT=$((CLIENT_COUNT - 1))
done
