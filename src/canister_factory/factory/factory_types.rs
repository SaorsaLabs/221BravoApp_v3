use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Default, CandidType, Deserialize, Serialize, Clone)]
pub struct Wasm {
    pub name: String,
    pub wasm: Vec<u8>,
    pub version: Option<String>,
}

#[derive(Default, CandidType, Deserialize, Serialize, Clone)]
pub struct WasmManager {
 available_wasms: Vec<Wasm>
}
impl WasmManager {

    pub fn clear_wasm_vec(&mut self, wasm_name: String) -> String {
        for wsm in self.available_wasms.iter_mut() {
            if wsm.name == wasm_name {
                wsm.wasm.clear();
                return String::from("WASM Vec Cleared");
            }
        }
        return String::from("Could not find WASM by name");
    }

    pub fn remove_wasm(&mut self, wasm_name: String) -> String {
        self.available_wasms.retain(|x: &Wasm| x.name != wasm_name);
        return String::from("Removed any matching Wasms");      
    }
    
    pub fn add_wasm(&mut self, wasm_vec: Vec<u8>, name: String, version: Option<String>) -> String {
        self.available_wasms.push(Wasm{
            name,
            wasm: wasm_vec,
            version,
        });
        return String::from("WASM added to Wasm Manager");
    }

    pub fn add_wasm_chunk(&mut self, mut chunk: Vec<u8>, wasm_name: String) -> String {
        for wsm in self.available_wasms.iter_mut() {
            if wsm.name == wasm_name {
                wsm.wasm.append(&mut chunk);
                return String::from("Chunk added to wasm vec");
            }
        }
        return String::from("Could not find WASM by name");
    }

    pub fn get_wasm(&self, wasm_name: String) -> Option<Vec<u8>> {
        for wsm in self.available_wasms.iter() {
            if wsm.name == wasm_name {
                return Some(wsm.wasm.clone())
            }
        }
        None
    }

    pub fn get_all_available_wasms(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for wsm in self.available_wasms.iter() {
            res.push(wsm.name.clone());
        }
        return res;
    }

    pub fn get_wasm_vec_length(&self, wasm_name: String) -> usize {
        for wsm in self.available_wasms.iter() {
            if wsm.name == wasm_name {
                return wsm.wasm.len();
            }
        }
        return 0_usize;
    }
}

pub enum InstallMode {
    Install,
    Reinstall,
    Upgrade
}