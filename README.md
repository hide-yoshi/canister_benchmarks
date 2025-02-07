# `canister_benchmarks`

This Repository contains code to reproduce the “Couldn't send message” error that occurs during an inter canister call.

## Structure

Create 10 canisters and make inter canister calls from the first to the second, from the second to the third, .... The inter canister call is made in the order from the first to the second, from the second to the third, ..., from the ninth to the tenth canister.
The inter canister call is executed with the number of parallelism specified by the argument of the function `call_server_update_parallel`.
Also, inter canister calls are executed across subnets.

## Deploy

```
./ic_deploy.sh
```

## Run

```
dfx canister call proxy1 --ic call_server_update_parallel 60
```
