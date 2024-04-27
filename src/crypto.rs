use secp256k1::{SecretKey, Message, sign, verify};

pub fn sign_message(sk: &SecretKey, message: &[u8]) -> Result<Vec<u8>, secp256k1::Error> {
    let msg = Message::from_slice(message)?;
    let (sig, _) = sign(&msg, sk);
    Ok(sig.serialize().to_vec())
}

pub fn verify_signature(pk: &[u8], signature: &[u8], message: &[u8]) -> Result<bool, secp256k1::Error> {
    let msg = Message::from_slice(message)?;
    let sig = secp256k1::Signature::from_der(signature)?;
    let pubkey = secp256k1::PublicKey::from_slice(pk)?;

    Ok(verify(&msg, &sig, &pubkey).is_ok())
}
