# !bin/sh
CANISTER_COUNT=10
cargo build --release --target wasm32-unknown-unknown --manifest-path src/proxy/Cargo.toml

candid-extractor target/wasm32-unknown-unknown/release/proxy.wasm > src/proxy/proxy.did
dfx deploy -y --ic proxy1 --subnet qdvhd-os4o2-zzrdw-xrcv4-gljou-eztdp-bj326-e6jgr-tkhuc-ql6v2-yqe
dfx deploy -y --ic proxy2 --subnet o3ow2-2ipam-6fcjo-3j5vt-fzbge-2g7my-5fz2m-p4o2t-dwlc4-gt2q7-5ae
dfx deploy -y --ic proxy3 --subnet qdvhd-os4o2-zzrdw-xrcv4-gljou-eztdp-bj326-e6jgr-tkhuc-ql6v2-yqe
dfx deploy -y --ic proxy4 --subnet o3ow2-2ipam-6fcjo-3j5vt-fzbge-2g7my-5fz2m-p4o2t-dwlc4-gt2q7-5ae
dfx deploy -y --ic proxy5 --subnet qdvhd-os4o2-zzrdw-xrcv4-gljou-eztdp-bj326-e6jgr-tkhuc-ql6v2-yqe
dfx deploy -y --ic proxy6 --subnet o3ow2-2ipam-6fcjo-3j5vt-fzbge-2g7my-5fz2m-p4o2t-dwlc4-gt2q7-5ae
dfx deploy -y --ic proxy7 --subnet qdvhd-os4o2-zzrdw-xrcv4-gljou-eztdp-bj326-e6jgr-tkhuc-ql6v2-yqe
dfx deploy -y --ic proxy8 --subnet o3ow2-2ipam-6fcjo-3j5vt-fzbge-2g7my-5fz2m-p4o2t-dwlc4-gt2q7-5ae
dfx deploy -y --ic proxy9 --subnet qdvhd-os4o2-zzrdw-xrcv4-gljou-eztdp-bj326-e6jgr-tkhuc-ql6v2-yqe
dfx deploy -y --ic proxy10 --subnet o3ow2-2ipam-6fcjo-3j5vt-fzbge-2g7my-5fz2m-p4o2t-dwlc4-gt2q7-5ae


for i in $(seq 1 $CANISTER_COUNT)
do
  CANISTER_NAME="proxy$i"
  NEXT_CANISTER_NAME="proxy$(($i+1))"
  if [ $i -ne $CANISTER_COUNT ]
  then
    dfx canister call $CANISTER_NAME set_server --ic "$(dfx canister id $NEXT_CANISTER_NAME --ic)"
  fi
done
