use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct KeyMap<'a>{
    path: String,
    depth: u8,
    private_key: String,
    fingerprint: &'a str,
    chaincode: String
}

impl<'a> KeyMap<'a>{

    pub fn new(
        path:String,depth:u8,
        private_key:String, fingerprint:&'a str,
        chaincode: String
    ) -> Self {
        Self { path, depth, private_key, fingerprint, chaincode }
    }

}