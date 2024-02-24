import tokenJSON from '../staticData/tokens.json';

export function millisToDate(epochMillis) {
	let t1, tDate;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	t1 = new Date(epochMillis);
	tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
	return tDate;
}

export function millisToTime(epochMillis) {
	let t1, tTime;
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	t1 = new Date(epochMillis);
	tTime = t1.toLocaleString('en-GB', options2); //
	return tTime;
}

export function datetimeToMillis(datetime, timezone) {
	let stLen;
	let lastChar;
	let dTime = datetime;
	let epochTime = 0;
	if (timezone == 'UTC') {
		stLen = datetime?.length ?? 0;
		lastChar = dTime[stLen - 1];
		if (lastChar != null) {
			if (lastChar != 'Z') dTime = dTime + 'Z';
			epochTime = Date.parse(dTime);
			return epochTime;
		} else {
			console.log("Error - Can't determine last charater");
		}
	}
}

export function combinePrincipalSubAccount(principal, subaccount) {
	let ret = `${principal}.${subaccount}`;
	return ret;
}

export function parsePrincipalSubAccountString(str) {
	const separatorIndex = str.indexOf('.');

	if (separatorIndex === -1) {
		// If no separator found, return an object with empty strings
		return { principal: '', subaccount: '' };
	}

	const principal = str.slice(0, separatorIndex);
	const subaccount = str.slice(separatorIndex + 1);

	return { principal, subaccount };
}

export function shortenString(str, firstChunk, endChunk) {
	if (str == undefined) return undefined;

	if (str.length < 15) {
		return str;
	} else {
		const firstchunk = str.slice(0, firstChunk);
		const lastchunk = str.slice(-endChunk);
		return `${firstchunk}....${lastchunk}`;
	}
}

export function getUniqueValues(array){
    array.sort();
    let keepers = [];
    let i;
    keepers[0] = array[0]; // 1st is always a keeper
    let LL = array.length;
    for(i = 1; i<LL; i++) {
        if(array[i] != array[i-1]) keepers.push(array[i]);
    }
    return keepers;
}

export async function processPromises(arrayOfPromises) {
    let responses = await Promise.all(arrayOfPromises);
    return responses;
}

export function getTokenData(TICKER){
	let data = tokenJSON;
	let len = data.length ?? 0;
	let retData; 
	let foundOne = false;
	for(let i = 0; i< len; i++){
		if(data[i].ticker == TICKER) {
			retData = data[i];
			foundOne = true;
			break;
		}
	}
	if (foundOne == true) return retData;
	else return "Could not find a matching token";
}

export function getAllTokenData(){
	let data = tokenJSON;
	return data;
}

export function parseTicker(ticker){
	let parts = ticker.split("/");
	return {base: parts[0], quote: parts[1]};
}

export function nanoToDate(epochNanoseconds){
	const milliseconds = epochNanoseconds / 1000000; // Convert nanoseconds to milliseconds
	const date = new Date(milliseconds);
	let isoDate = date.toISOString();
	let opDate = isoDate;
	let ind = isoDate.indexOf("T");
	let short = isoDate.substring(ind + 1, ind + 6);
	let dateOnly = isoDate.substring(0,10);
	let rev1 =dateOnly.split("-");
	let revDateOnly = `${rev1[2]}-${rev1[1]}-${rev1[0]}`;
	let ret = {
		fullDateTime: opDate,
		shortTime: short,
		dateOnly: revDateOnly,
		UTCTimestampSecs: milliseconds/1000, // for lightweight charts
	}
	return ret;
} // UTCTimestamp 