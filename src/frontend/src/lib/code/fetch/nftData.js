import { parsePrincipalSubAccountString } from '../utils.js';
import { DEFAULT_SUBACCOUNT } from '../constants.js';

export async function getNFTcount(searchInput){
    if (searchInput == "" || searchInput == null || searchInput == undefined) return 0;
    if (searchInput.includes(".") && searchInput.includes("-")){
        // icrc account
        let parsed = parsePrincipalSubAccountString(searchInput);
        if (parsed.subaccount == DEFAULT_SUBACCOUNT){
            try {
                let settings = { method: "Get" };
                let url = `https://api.nftgeek.app/api/221bravo/principal/${parsed.principal}/summary`;
                const response = await fetch(url, settings);
                const data = await response.json();
                let ret = data?.summary?.ownedNfts ?? 0;
                return ret;
            } catch (error) {
                return 0;
            }
        } else {
            return 0;
        }
    } else if (searchInput.includes("-")){
        // principal only
        try {
            let settings = { method: "Get" };
            let url = `https://api.nftgeek.app/api/221bravo/principal/${searchInput}/summary`;
            const response = await fetch(url, settings);
            const data = await response.json();
            let ret = data?.summary?.ownedNfts ?? 0;
            return ret;
        } catch (error) {
            return 0;
        }
    } else {
        // ICP-og style account
        try {
            let settings = { method: "Get" };
            let url = `https://api.nftgeek.app/api/221bravo/accountIdentifier/${searchInput}/summary`;
            const response = await fetch(url, settings);
            const data = await response.json();
            let ret = data?.summary?.ownedNfts ?? 0;
            return ret;
        } catch (error) {
            return 0;
        }
    }
}

export function nftGeekURLConstructor(searchInput){
    if (searchInput == "" || searchInput == null || searchInput == undefined) return 0;
    if (searchInput.includes(".") && searchInput.includes("-")){
        // icrc account
        let parsed = parsePrincipalSubAccountString(searchInput);
        let url = `https://nftgeek.app/holder/${parsed.principal}/summary`;
        return url;
    } else if (searchInput.includes("-")){
        // principal
        let url = `https://nftgeek.app/holder/${searchInput}/summary`;
        return url;
    } else {
        // icp-og style
        let url = `https://nftgeek.app/holder/${searchInput}/summary`;
        return url;
    }
}