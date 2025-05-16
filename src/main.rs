
use std::io::{self, BufRead};
use alloy_consensus::{transaction::eip4844, EthereumTxEnvelope, Transaction};
use alloy_primitives::{ Address};
use alloy_rlp::Decodable;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[^0-9A-Za-z]").unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let txt = line.unwrap();
        if txt.starts_with("#"){
            continue
        }
        let txt = txt.trim_start_matches("0x").trim_start_matches("0X");
        let txt = re.replace_all(txt, "").into_owned();
        let result = match hex::decode(txt){
            Err(_) => {
                println!("err: hex decoding error");
                continue
            },
            Ok(bytes) => { bytes }
        };
        match parse_sender(&mut &result[..]) {
            Ok(sender )=>{
                println!("{sender:?}");
            }
            Err(txt) =>{
                println!("err: {}", txt);
            }
        }
    }
}

fn parse_sender(data: &mut &[u8]) -> Result<Address, String> {
    use alloy_consensus::transaction::SignerRecoverable;
    let envelope: EthereumTxEnvelope<eip4844::TxEip4844> = EthereumTxEnvelope::decode(data).map_err(|e| e.to_string())?;
    if let Some(chainid) =  envelope.chain_id() {
        if chainid != 1 {
            return Err(format!("chain id mismatch, have {} want 1", chainid));
        }
    }
    envelope.recover_signer().map_err(|e| e.to_string())
}
