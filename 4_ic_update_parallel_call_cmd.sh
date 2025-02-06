# !bin/sh

COUNT=$1
CANISTER_NAME=$2

## function: write data to file
write_count_and_result_to_file() {
    COUNT=$1
    RESULT=$2
    ## trim double quotes in the result
    RESULT=$(echo $RESULT | tr -d '"')
    ## format: csv
    echo "$COUNT,$RESULT,$CANISTER_NAME" >> update_call_parallel.csv
}
RESULT=$(dfx canister call $CANISTER_NAME call_server_update_parallel $COUNT --ic --output json)
echo $RESULT
write_count_and_result_to_file $COUNT $RESULT $CANISTER_NAME

