# !bin/sh

TOTAL=20
COUNT=0
CLIENT_COUNT=10

execute_update_call() {
    COUNT=$1
    CLIENT=$2
    RESULT=$(dfx canister call $CLIENT call_server_update $COUNT --ic --output json)
    echo $RESULT
    write_count_and_result_to_file $COUNT $RESULT 
}


## function: write data to file
write_count_and_result_to_file() {
    COUNT=$1
    RESULT=$2
    CLIENT_COUNT=$3
    ## trim double quotes in the result
    RESULT=$(echo $RESULT | tr -d '"')
    ## format: csv
    echo "$COUNT,$RESULT" >> update_call.csv
}




while [ $COUNT -lt $TOTAL ]; do
    # 非同期で同時に複数のクライアントからアップデートを呼び出す
    # get the result of the call
    while [ $CLIENT_COUNT -gt 0 ]; do
        if [ $COUNT -eq 0 ]; then
            CLIENT=client
        else
            CLIENT=client$(($CLIENT_COUNT+1))
        fi
        execute_update_call $COUNT $CLIENT &
        CLIENT_COUNT=$((CLIENT_COUNT - 1))
    done

    RESULT=$(dfx canister call client call_server_update $COUNT --ic --output json)
    echo $RESULT
    write_count_and_result_to_file $COUNT $RESULT $CLIENT_COUNT
    COUNT=$((COUNT + 1))
done

