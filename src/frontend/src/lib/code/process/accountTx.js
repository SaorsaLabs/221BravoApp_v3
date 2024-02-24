
// Processed TX format to Account Table Format
export function processAccountTX(inputArray, searchedAC, token){
    // Input
    // block
    // from_account
    // spender
    // to_account
    // tx_fee
    // tx_time
    // tx_type
    // tx_value
    // toUserName
    // fromUserName
    // fromGlobalName
    // toGlobalName

    let i;
    let inptLen = inputArray?.length ?? 0;
    if (inptLen == 0) return [];
    let direction, linkedAC;
    let count = 1;
    let outputAR = [];
    let tUN, fUN, tGN, fGN;
    for(i=0; i<inptLen; i++){
        fUN = inputArray[i]?.fromUserName ? inputArray[i].fromUserName : null;
        tUN = inputArray[i]?.toUserName ? inputArray[i].toUserName : null;
        fGN = inputArray[i]?.fromGlobalName ? inputArray[i].fromGlobalName : null;
        tGN = inputArray[i]?.toGlobalName ? inputArray[i].toGlobalName : null;

        if(inputArray[i].from_account == searchedAC){
            direction = "out";
            linkedAC = inputArray[i].to_account;
        } else {
            direction = "in";
            linkedAC = inputArray[i].from_account;
        }
        outputAR.push({
            block: Number(inputArray[i].block),
            count,
            time: inputArray[i].tx_time,
            token,
            linkedAC,
            direction,
            value: Number(inputArray[i].tx_value),
            type: inputArray[i].tx_type,
            fee: inputArray[i].tx_fee,
            fromGlobalName: fGN,
            toGlobalName: tGN,
            fromUserName: fUN,
            toUserName: tUN
        });
        count++;
    }// for
    return outputAR;
}

// Processed TX format to Account Table Format (Multiple Ledgers)
export function processMultiAccountTX(inputArray, searchedAC, token, decimals){
    // Input
    // block
    // from_account
    // spender
    // to_account
    // tx_fee
    // tx_time
    // tx_type
    // tx_value
    // toUserName
    // fromUserName
    // fromGlobalName
    // toGlobalName

    let i;
    let decimalPower = Math.pow(10, decimals);
    let inptLen = inputArray?.length ?? 0;
    if (inptLen == 0) return [];
    let direction, linkedAC;
    let count = 1;
    let outputAR = [];
    let tUN, fUN, tGN, fGN;
    for(i=0; i<inptLen; i++){
        fUN = inputArray[i]?.fromUserName ? inputArray[i].fromUserName : null;
        tUN = inputArray[i]?.toUserName ? inputArray[i].toUserName : null;
        fGN = inputArray[i]?.fromGlobalName ? inputArray[i].fromGlobalName : null;
        tGN = inputArray[i]?.toGlobalName ? inputArray[i].toGlobalName : null;

        if(inputArray[i].from_account == searchedAC){
            direction = "out";
            linkedAC = inputArray[i].to_account;
        } else {
            direction = "in";
            linkedAC = inputArray[i].from_account;
        }
        outputAR.push({
            block: Number(inputArray[i].block),
            count,
            time: inputArray[i].tx_time,
            token,
            linkedAC,
            direction,
            value: Number(inputArray[i].tx_value)/ decimalPower,
            type: inputArray[i].tx_type,
            fee: inputArray[i].tx_fee,
            fromGlobalName: fGN,
            toGlobalName: tGN,
            fromUserName: fUN,
            toUserName: tUN
        });
        count++;
    }// for
    return outputAR;
}