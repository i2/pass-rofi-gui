use anyhow::anyhow;
use anyhow::Error;
use libreauth::oath::TOTPBuilder;

pub fn calculate_otp(secret: &str) -> Result<String, Error> {
    Ok(TOTPBuilder::new()
        .base32_key(&secret.to_owned())
        .finalize()
        .map_err(|err| anyhow!("libreauth error: {:?}", err))?
        .generate())
}
