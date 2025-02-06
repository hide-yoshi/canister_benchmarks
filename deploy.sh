# !bin/sh
cargo build --release --target wasm32-unknown-unknown

candid-extractor target/wasm32-unknown-unknown/release/client.wasm > src/client/client.did
candid-extractor target/wasm32-unknown-unknown/release/server.wasm > src/server/server.did
dfx deploy -y

CANISTER_ID=$(yq e '.server.local' .dfx/local/canister_ids.json)

dfx canister call client set_server $CANISTER_ID
