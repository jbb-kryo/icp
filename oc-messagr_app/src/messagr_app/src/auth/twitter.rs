use crate::{AuthConfig, Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha1;
use rand::{thread_rng, Rng};
use base64::{Engine as _, engine::general_purpose};
use std::time::{SystemTime, UNIX_EPOCH};

// Twitter uses OAuth 1.0a which requires HMAC-SHA1 signatures
type HmacSha1 = Hmac<Sha1>;

pub fn validate_auth(auth_config: &AuthConfig) -> Result<()> {
    // Check that OAuth token is provided
    if auth_config.token.is_empty() {
        return Err(Error::InvalidParameters("Twitter OAuth token is required".to_string()));
    }
    
    // Check that consumer key is provided
    if auth_config.api_key.is_none() {
        return Err(Error::InvalidParameters("Twitter API key is required".to_string()));
    }
    
    // Check that consumer secret is provided
    if auth_config.api_secret.is_none() {
        return Err(Error::InvalidParameters("Twitter API secret is required".to_string()));
    }
    
    // Twitter requires a token secret alongside the token
    // In a real implementation, we'd store this as part of the token or in a separate field
    if !auth_config.token.contains(":") {
        return Err(Error::InvalidParameters(
            "Twitter OAuth token must include token secret in format 'token:secret'".to_string()
        ));
    }
    
    Ok(())
}

// Generate OAuth 1.0a signature for Twitter API requests
pub fn generate_oauth_signature(
    auth_config: &AuthConfig,
    method: &str,
    url: &str,
    params: &[(String, String)],
) -> Result<String> {
    // Extract API key (consumer key)
    let consumer_key = auth_config.api_key.clone()
        .ok_or_else(|| Error::InvalidParameters("Twitter API key is required".to_string()))?;
    
    // Extract API secret (consumer secret)
    let consumer_secret = auth_config.api_secret.clone()
        .ok_or_else(|| Error::InvalidParameters("Twitter API secret is required".to_string()))?;
    
    // Extract token and token secret
    let token_parts: Vec<&str> = auth_config.token.split(':').collect();
    if token_parts.len() != 2 {
        return Err(Error::InvalidParameters(
            "Twitter OAuth token must include token secret in format 'token:secret'".to_string()
        ));
    }
    let token = token_parts[0].to_string();
    let token_secret = token_parts[1].to_string();
    
    // Generate OAuth nonce (random string)
    let nonce = generate_nonce();
    
    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::InternalError(format!("Failed to get timestamp: {}", e)))?
        .as_secs()
        .to_string();
    
    // Create OAuth parameters
    let mut oauth_params = vec![
        ("oauth_consumer_key".to_string(), consumer_key),
        ("oauth_nonce".to_string(), nonce),
        ("oauth_signature_method".to_string(), "HMAC-SHA1".to_string()),
        ("oauth_timestamp".to_string(), timestamp),
        ("oauth_token".to_string(), token),
        ("oauth_version".to_string(), "1.0".to_string()),
    ];
    
    // Combine with request parameters
    let mut all_params = oauth_params.clone();
    all_params.extend(params.iter().cloned());
    
    // Sort parameters lexicographically
    all_params.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Create parameter string
    let param_string = all_params
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<String>>()
        .join("&");
    
    // Create signature base string
    let signature_base = format!(
        "{}&{}&{}",
        method.to_uppercase(),
        url_encode(url),
        url_encode(&param_string)
    );
    
    // Create signing key
    let signing_key = format!("{}&{}", url_encode(&consumer_secret), url_encode(&token_secret));
    
    // Create HMAC-SHA1 signature
    let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())
        .map_err(|e| Error::InternalError(format!("Failed to create HMAC: {}", e)))?;
    
    mac.update(signature_base.as_bytes());
    let result = mac.finalize();
    let signature = general_purpose::STANDARD.encode(result.into_bytes());
    
    // Add signature to OAuth parameters
    oauth_params.push(("oauth_signature".to_string(), signature.clone()));
    
    // Generate Authorization header
    let auth_header = oauth_params
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, url_encode(v)))
        .collect::<Vec<String>>()
        .join(", ");
    
    Ok(format!("OAuth {}", auth_header))
}

// Generate random OAuth nonce
fn generate_nonce() -> String {
    let mut rng = thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
    general_purpose::STANDARD.encode(&random_bytes)
}

// URL encode a string according to OAuth 1.0a specs
fn url_encode(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~' => result.push(c),
            _ => {
                let bytes = c.to_string().into_bytes();
                for byte in bytes {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}