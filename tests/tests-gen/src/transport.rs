use anyhow::{anyhow, Result};
use byteorder::WriteBytesExt;
use byteorder::LE;
use hex_literal::hex;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Write;

fn handle_error_code(code: u16) -> Result<()> {
    match code {
        0x9000 => Ok(()),
        0x6D02 => Err(anyhow!("Zcash Application NOT OPEN")),
        0x6985 => Err(anyhow!("Tx REJECTED by User")),
        0x5515 => Err(anyhow!("Ledger is LOCKED")),
        _ => Err(anyhow!("Ledger device returned error code {:#06x}", code)),
    }
}

const TEST_SERVER_IP: Option<&'static str> = option_env!("LEDGER_IP");

fn apdu(data: &[u8]) -> Result<Vec<u8>> {
    let client = Client::new();
    let response = client
        .post(&format!("http://{}:5000/apdu", TEST_SERVER_IP.unwrap()))
        .body(format!("{{\"data\": \"{}\"}}", hex::encode(data)))
        .send()?;
    let response_body: Value = response.json()?;
    let data = response_body["data"]
        .as_str()
        .ok_or(anyhow!("No data field"))?;
    let data = hex::decode(data)?;
    let error_code = u16::from_be_bytes(data[data.len() - 2..].try_into().unwrap());
    handle_error_code(error_code)?;
    Ok(data[..data.len() - 2].to_vec())
}

pub fn ledger_init() -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.clear();
    bb.write_all(&hex!("E005000000"))?;
    apdu(&bb)?;

    Ok(())
}

pub fn ledger_init_tx() -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E010000000"))?;
    let main_seed = apdu(&bb)?;
    Ok(main_seed)
}

pub fn ledger_set_stage(stage: u8) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E011"))?;
    bb.write_u8(stage)?;
    bb.write_all(&hex!("0000"))?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_set_transparent_merkle_proof(
    header_digest: &[u8],
    prevouts_digest: &[u8],
    pubscripts_digest: &[u8],
    sequences_digest: &[u8],
) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E012000080"))?;
    bb.write_all(header_digest)?;
    bb.write_all(prevouts_digest)?;
    bb.write_all(pubscripts_digest)?;
    bb.write_all(sequences_digest)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_set_sapling_merkle_proof(
    spends_digest: &[u8],
    memos_digest: &[u8],
    outputs_nc_digest: &[u8],
) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E013000060"))?;
    bb.write_all(spends_digest)?;
    bb.write_all(memos_digest)?;
    bb.write_all(outputs_nc_digest)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_set_orchard_merkle_proof(
    anchor: &[u8],
    memos_digest: &[u8],
    outputs_nc_digest: &[u8],
) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E014000060"))?;
    bb.write_all(anchor)?;
    bb.write_all(memos_digest)?;
    bb.write_all(outputs_nc_digest)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_add_t_input(amount: u64) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E015000008"))?;
    bb.write_u64::<LE>(amount)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_add_t_output(amount: u64, address_type: u8, address: &[u8]) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E01600001D"))?;
    bb.write_u64::<LE>(amount)?;
    bb.write_u8(address_type)?;
    bb.write_all(address)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_add_s_output(
    amount: u64,
    epk: &[u8],
    address: &[u8],
    enc_compact: &[u8],
    rseed: &[u8],
) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0170000A7"))?;
    bb.write_all(address)?;
    bb.write_u64::<LE>(amount)?;
    bb.write_all(epk)?;
    bb.write_all(enc_compact)?;
    bb.write_all(rseed)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_add_o_action(
    nf: &[u8],
    amount: u64,
    epk: &[u8],
    address: &[u8],
    enc_compact: &[u8],
    rseed: &[u8],
) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0180000C7"))?;
    bb.write_all(nf)?;
    bb.write_all(address)?;
    bb.write_u64::<LE>(amount)?;
    bb.write_all(epk)?;
    bb.write_all(enc_compact)?;
    bb.write_all(rseed)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_set_net_sapling(net: i64) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E019000008"))?;
    bb.write_i64::<LE>(net)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_set_net_orchard(net: i64) -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E01A000008"))?;
    bb.write_i64::<LE>(net)?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_confirm_fee() -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E01B000000"))?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_get_sighash() -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E020000000"))?;
    let sighash = apdu(&bb)?;
    Ok(sighash)
}

pub fn ledger_sign_transparent(txin_digest: &[u8]) -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E022000020"))?;
    bb.write_all(txin_digest)?;
    let signature = apdu(&bb)?;
    Ok(signature)
}

pub fn ledger_sign_sapling() -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E023000000"))?;
    let signature = apdu(&bb)?;
    Ok(signature)
}

pub fn ledger_sign_orchard() -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E024000000"))?;
    let signature = apdu(&bb)?;
    Ok(signature)
}

pub fn ledger_end_tx() -> Result<()> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E030000000"))?;
    apdu(&bb)?;
    Ok(())
}

pub fn ledger_cmu(data: &[u8]) -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0800000"))?;
    bb.write_u8(data.len() as u8)?;
    bb.write_all(data)?;
    let cmu = apdu(&bb)?;
    Ok(cmu)
}

pub fn ledger_jubjub_hash(data: &[u8]) -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0810000"))?;
    bb.write_u8(data.len() as u8)?;
    bb.write_all(data)?;
    let cmu = apdu(&bb)?;
    Ok(cmu)
}

pub fn ledger_pedersen_hash(data: &[u8]) -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0820000"))?;
    bb.write_u8(data.len() as u8)?;
    bb.write_all(data)?;
    let cmu = apdu(&bb)?;
    Ok(cmu)
}

pub fn ledger_test_math(i: u8) -> Result<Vec<u8>> {
    let mut bb: Vec<u8> = vec![];
    bb.write_all(&hex!("E0FF"))?;
    bb.write_u8(i)?;
    bb.write_all(&hex!("0000"))?;
    let res = apdu(&bb)?;
    Ok(res)
}