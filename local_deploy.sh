# !bin/sh
CANISTER_COUNT=10
cargo build --release --target wasm32-unknown-unknown --manifest-path src/proxy/Cargo.toml

candid-extractor target/wasm32-unknown-unknown/release/proxy.wasm > src/proxy/proxy.did
dfx deploy -y

for i in $(seq 1 $CANISTER_COUNT)
do
  CANISTER_NAME="proxy$i"
  NEXT_CANISTER_NAME="proxy$(($i+1))"
  if [ $i -ne $CANISTER_COUNT ]
  then
    dfx canister call $CANISTER_NAME set_server "$(dfx canister id $NEXT_CANISTER_NAME)"
  fi
done
