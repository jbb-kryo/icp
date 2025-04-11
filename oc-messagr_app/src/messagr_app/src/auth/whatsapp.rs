use crate::{AuthConfig, Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn validate_auth(auth_config: &AuthConfig) -> Result<()> {
    // Check that token is provided (WhatsApp Business API token)
    if auth_config.token.is_empty() {
        return Err(Error::InvalidParameters("WhatsApp access token is required".to_string()));
    }
    
    // Check that phone number ID is provided (as API key)
    if auth_config.api_key.is_none() {
        return Err(Error::InvalidParameters("WhatsApp phone number ID is required".to_string()));
    }
    
    // Check that app secret is provided (for webhook verification)
    if auth_config.api_secret.is_none() {
        return Err(Error::InvalidParameters("WhatsApp app secret is required".to_string()));
    }
    
    Ok(())
}

// Verify a webhook request from WhatsApp
pub fn verify_webhook(
    auth_config: &AuthConfig,
    mode: &str,
    token: &str,
    challenge: &str,
) -> Result<String> {
    // WhatsApp Cloud API uses the same webhook verification as Facebook
    if mode != "subscribe" {
        return Err(Error::InvalidParameters("Invalid mode parameter".to_string()));
    }
    
    // The token should match the verification token registered with the app
    // In a real implementation, we'd store this value separately
    let expected_token = "WEBHOOK_VERIFY_TOKEN"; // This would be configurable
    
    if token != expected_token {
        return Err(Error::InvalidParameters("Invalid verification token".to_string()));
    }
    
    // If everything is valid, return the challenge
    Ok(challenge.to_string())
}

// Verify a webhook payload from WhatsApp using the signature
pub fn verify_webhook_signature(
    auth_config: &AuthConfig,
    signature: &str,
    payload: &[u8],
) -> Result<bool> {
    let app_secret = auth_config.api_secret.clone()
        .ok_or_else(|| Error::InvalidParameters("WhatsApp app secret is required".to_string()))?;
    
    // The signature header is in format "sha256=..."
    let signature = signature.strip_prefix("sha256=")
        .ok_or_else(|| Error::InvalidParameters("Invalid signature format".to_string()))?;
    
    // Convert hex signature to bytes
    let signature_bytes = hex_to_bytes(signature)
        .map_err(|e| Error::InvalidParameters(format!("Invalid signature: {}", e)))?;
    
    // Create an HMAC-SHA256 using the app secret
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(app_secret.as_bytes())
        .map_err(|e| Error::InternalError(format!("Failed to create HMAC: {}", e)))?;
    
    // Update with payload
    mac.update(payload);
    
    // Verify the signature
    mac.verify_slice(&signature_bytes)
        .map(|_| true)
        .map_err(|_| Error::InvalidParameters("Invalid signature".to_string()))
}

// Convert hexadecimal string to bytes
fn hex_to_bytes(hex: &str) -> std::result::Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i+2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|e| format!("Invalid hex byte '{}': {}", byte_str, e))?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}