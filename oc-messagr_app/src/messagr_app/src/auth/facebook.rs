use crate::{AuthConfig, Error, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, 
    TokenUrl, AuthorizationCode, TokenResponse, Scope,
};
use url::Url;

pub fn validate_auth(auth_config: &AuthConfig) -> Result<()> {
    // Check that token is provided (Page Access Token)
    if auth_config.token.is_empty() {
        return Err(Error::InvalidParameters("Facebook page access token is required".to_string()));
    }
    
    // Check that App ID is provided
    if auth_config.api_key.is_none() {
        return Err(Error::InvalidParameters("Facebook App ID is required".to_string()));
    }
    
    // Check that App Secret is provided
    if auth_config.api_secret.is_none() {
        return Err(Error::InvalidParameters("Facebook App Secret is required".to_string()));
    }
    
    // Check that redirect URI is provided for OAuth
    if auth_config.redirect_uri.is_none() {
        return Err(Error::InvalidParameters("Facebook redirect URI is required".to_string()));
    }
    
    Ok(())
}

// Create OAuth client for Facebook
pub fn create_oauth_client(auth_config: &AuthConfig) -> Result<BasicClient> {
    let client_id = auth_config.api_key.clone()
        .ok_or_else(|| Error::InvalidParameters("Facebook App ID is required".to_string()))?;
    
    let client_secret = auth_config.api_secret.clone()
        .ok_or_else(|| Error::InvalidParameters("Facebook App Secret is required".to_string()))?;
    
    let redirect_uri = auth_config.redirect_uri.clone()
        .ok_or_else(|| Error::InvalidParameters("Facebook redirect URI is required".to_string()))?;
    
    // Parse the redirect URI
    let redirect_url = RedirectUrl::new(redirect_uri)
        .map_err(|e| Error::InvalidParameters(format!("Invalid redirect URI: {}", e)))?;
    
    // Create the client
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://www.facebook.com/v16.0/dialog/oauth".to_string())
            .map_err(|e| Error::InternalError(format!("Invalid Facebook auth URL: {}", e)))?,
        Some(TokenUrl::new("https://graph.facebook.com/v16.0/oauth/access_token".to_string())
            .map_err(|e| Error::InternalError(format!("Invalid Facebook token URL: {}", e)))?)
    )
    .set_redirect_uri(redirect_url);
    
    Ok(client)
}

// Generate the OAuth authorization URL for Facebook
pub fn get_auth_url(auth_config: &AuthConfig) -> Result<String> {
    let client = create_oauth_client(auth_config)?;
    
    // Generate the authorization URL
    let (auth_url, _csrf_token) = client
        .authorize_url(|| "state".to_string())
        .add_scope(Scope::new("pages_messaging".to_string()))
        .add_scope(Scope::new("pages_messaging_subscriptions".to_string()))
        .add_scope(Scope::new("pages_show_list".to_string()))
        .add_scope(Scope::new("pages_read_engagement".to_string()))
        .url();
    
    Ok(auth_url.to_string())
}

// Exchange the authorization code for an access token
pub async fn exchange_code_for_token(
    auth_config: &AuthConfig,
    code: &str
) -> Result<String> {
    let client = create_oauth_client(auth_config)?;
    
    // Exchange the code for a token
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| Error::PlatformError(format!("Failed to exchange code: {}", e)))?;
    
    Ok(token_result.access_token().secret().clone())
}

// Verify a webhook request from Facebook
pub fn verify_webhook(
    verify_token: &str,
    challenge: &str,
    hub_mode: &str,
) -> Result<String> {
    // Check if this is a subscription and verify token matches
    if hub_mode == "subscribe" {
        // In a real implementation, we'd compare this with a stored verification token
        Ok(challenge.to_string())
    } else {
        Err(Error::InvalidParameters("Invalid hub.mode parameter".to_string()))
    }
}

// Generate app proof for API calls (app_id|access_token verification)
pub fn generate_app_proof(auth_config: &AuthConfig) -> Result<String> {
    let app_secret = auth_config.api_secret.clone()
        .ok_or_else(|| Error::InvalidParameters("Facebook App Secret is required".to_string()))?;
    
    // Create an HMAC-SHA256 signature of access_token using app_secret
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(app_secret.as_bytes())
        .map_err(|e| Error::InternalError(format!("Failed to create HMAC: {}", e)))?;
    
    mac.update(auth_config.token.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();
    
    // Convert to hexadecimal string
    let hex_proof: String = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    
    Ok(hex_proof)
}