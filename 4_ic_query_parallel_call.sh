# !bin/sh


TOTAL=20
COUNT=0
## function: write data to file
write_count_and_result_to_file() {
    COUNT=$1
    RESULT=$2
    ## trim double quotes in the result
    RESULT=$(echo $RESULT | tr -d '"')
    ## format: csv
    echo "$COUNT,$RESULT" >> query_call_parallel.csv
}
while [ $COUNT -lt $TOTAL ]; do
    # get the result of the call
    RESULT=$(dfx canister call client call_server_query_parallel $COUNT --ic --output json)
    echo $RESULT
    write_count_and_result_to_file $COUNT $RESULT
    COUNT=$((COUNT + 1))

done

