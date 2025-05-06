#![no_main]

use libfuzzer_sys::fuzz_target;
use alloy_consensus::{transaction::eip4844, EthereumTxEnvelope};
use alloy_primitives::{Address};
use alloy_rlp::Decodable;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    _ = parse_sender(&mut &data[..])
});

fn parse_sender(data: &mut &[u8])  -> Result<Address, String> {
    let envelope: EthereumTxEnvelope<eip4844::TxEip4844> = EthereumTxEnvelope::decode(data).map_err(|e| e.to_string())?;
    envelope.recover_signer().map_err(|e| e.to_string())
}
