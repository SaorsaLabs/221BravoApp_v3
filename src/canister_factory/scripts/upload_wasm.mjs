import { createReadStream, createWriteStream, existsSync } from 'node:fs';
import { open, readFile, rm } from 'node:fs/promises';
import { createGzip } from 'node:zlib';
import { icActor } from './icAgent.mjs';
import { factoryIDL } from './IDL/factory_canister.mjs';
import { getIdentity } from './icAgent.mjs';


// SOURCE & CREDIT - Juno Build
// https://github.com/junobuild/juno/blob/main/scripts/code.utils.mjs 

export const loadLocalWasm = async (type) => {
	const buffer = await readFile(`${process.cwd()}/.dfx/local/canisters/${type}/${type}.wasm`);
	return [...new Uint8Array(buffer)];
};

export const loadGzippedWasm = async (destination) => {
	const buffer = await readFile(destination);
	return [...new Uint8Array(buffer)];
};

export const gzipAndLoadLocalWasm = async (type) => {
	const source = `${process.cwd()}/.dfx/local/canisters/${type}/${type}.wasm`;
	const destination = `${source}.gz`;

	if (existsSync(destination)) {
		await rm(destination);
	}

	await gzipFile({ source, destination });

	return await loadGzippedWasm(destination);
};

const gzipFile = async ({ source, destination }) =>
	await new Promise((resolve, reject) => {
		const sourceStream = createReadStream(source);

		const destinationStream = createWriteStream(destination);

		const gzip = createGzip();

		sourceStream.pipe(gzip).pipe(destinationStream);

		destinationStream.on('close', () => {
			resolve(destination);
		});
		destinationStream.on('error', reject);
	});

export const readVersion = async (type) => {
	const file = await open(`${process.cwd()}/src/${type}/Cargo.toml`);

	try {
		for await (const line of file.readLines()) {
			let version = line.match(/version = "(.*)"/)?.[1];

			if (version !== undefined) {
				return version;
			}
		}

		return undefined;
	} finally {
		await file.close();
	}
};

///  ------------- CUSTOM UPLOAD CODE  
async function uploadWasmToFactory(canister, canisterWasm, wasmName, version, local_network){

    let id = getIdentity();
    let wasm = await gzipAndLoadLocalWasm(canisterWasm);
    let chunkSize = 200000;
    let actor = await icActor(canister, factoryIDL, id, local_network);
    let wasmLen = wasm.length;
    console.log("WASM LEN :: ", wasmLen);

    if (wasmLen> chunkSize ){    
        const remainder = wasmLen % chunkSize;
        let div = Math.floor(wasmLen / chunkSize);
        let loops = (remainder > 0) ? div+1 : div;

        for (let i = 0; i < loops; i++) {
            // first chunk
            if (i == 0){
                let chunks = wasm.slice(0, chunkSize);
                let call = await actor.add_wasm(chunks, wasmName, [version]);
                console.log("First Chunk :: ", call);
            } else {
            // follow on chunks
                let chunks = wasm.slice(i*chunkSize, ((i*chunkSize)+chunkSize));
                let call = await actor.add_wasm_chunk(chunks, wasmName);
                console.log("Added Chunk :: ", call);
            }
        }
		// check
		let callCheck = await actor.get_wasm_length(wasmName);
		if (callCheck == wasmLen) {
			console.log("Upload Complete");
		} else {
			console.log("ERROR - Uploaded wasm size != local wasm size");
		}
    } else {
        // install in one go. 
        await actor.add_wasm(wasm, wasmName, [version]);
		// check
		let callCheck = await actor.get_wasm_length(wasmName);
		if (callCheck == wasmLen) {
			console.log("Upload Complete");
		} else {
			console.log("ERROR - Uploaded wasm size != local wasm size", " Uploaded - ", callCheck);
		}
    }
}	
					// canister,                    canisterWasm,    wasmName,  version,  local_network
uploadWasmToFactory("okuxs-wiaaa-aaaak-qidcq-cai", "tracking_canister", "TRACK_011", "0.1.1", false); 