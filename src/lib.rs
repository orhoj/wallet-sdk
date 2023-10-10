use key_derivation::{ConcordiumHdWallet, Net};
use anyhow::{bail, Result};
use hex;
use std::os::raw::c_char;
use std::{
    convert::TryInto,
    ffi::{CString},
};

pub type JsonString = String;
pub type HexString = String;
pub type Base58String = String;

fn get_net(net: &str) -> Result<Net> {
    Ok(match net {
        "Mainnet" => Net::Mainnet,
        "Testnet" => Net::Testnet,
        _ => bail!("Unknown net"),
    })
}

fn get_wallet(seed_as_hex: HexString, raw_net: &str) -> Result<ConcordiumHdWallet> {
    let seed_decoded = hex::decode(&seed_as_hex)?;
    let seed: [u8; 64] = match seed_decoded.try_into() {
        Ok(s) => s,
        Err(_) => bail!("The provided seed {} was not 64 bytes", seed_as_hex),
    };

    let net = get_net(raw_net)?;
    let wallet = ConcordiumHdWallet { seed, net };
    Ok(wallet)
}

pub fn get_account_signing_key_aux(
    seed_as_hex: HexString,
    raw_net: &str,
    identity_provider_index: u32,
    identity_index: u32,
    credential_counter: u32,
) -> Result<String> {
    let wallet = get_wallet(seed_as_hex, raw_net)?;
    let key = wallet.get_account_signing_key(
        identity_provider_index,
        identity_index,
        credential_counter,
    )?;
    Ok(hex::encode(key.as_bytes()))
}

/// This function does not check that the flag pointer is not null.
unsafe fn signal_error(err_msg: String) -> *mut c_char {
    CString::new(err_msg)
        .expect("Error message string should be non-zero and utf8.")
        .into_raw()
}

unsafe fn encode_response(response: anyhow::Result<String>) -> *mut c_char {
    match response {
        Ok(s) => {
            let cstr: CString = {
                match CString::new(s) {
                    Ok(s) => s,
                    Err(e) => {
                        return signal_error(format!("Could not encode response: {}", e))
                    }
                }
            };
            cstr.into_raw()
        }
        Err(e) => signal_error(format!("Could not produce response: {}", e)),
    }
}

#[no_mangle]
pub extern "C" fn get_key() -> *mut c_char { //here is the trick
    let seed = "4755521eabc62968cb8f0b943b45cc72e76fb14d1554213b78209589b803a3cec052ec3496fe71982f180b224419a9203add89345914661521f8aa3a5189d6e5".to_owned();
    let signing_key = get_account_signing_key_aux(seed, "Testnet", 0, 0, 0);

    unsafe { encode_response(signing_key) }
}
