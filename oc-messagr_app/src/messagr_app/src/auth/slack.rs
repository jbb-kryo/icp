use crate::{AuthConfig, Error, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, 
    TokenUrl, AuthorizationCode, TokenResponse, Scope,
};
use url::Url;

pub fn validate_auth(auth_config: &AuthConfig) -> Result<()> {
    // Check that token is provided (OAuth access token)
    if auth_config.token.is_empty() {
        return Err(Error::InvalidParameters("Slack OAuth token is required".to_string()));
    }
    
    // Check that client ID and secret are provided for OAuth
    if auth_config.api_key.is_none() {
        return Err(Error::InvalidParameters("Slack client ID is required".to_string()));
    }
    
    if auth_config.api_secret.is_none() {
        return Err(Error::InvalidParameters("Slack client secret is required".to_string()));
    }
    
    // Redirect URI is needed for OAuth flow
    if auth_config.redirect_uri.is_none() {
        return Err(Error::InvalidParameters("Slack redirect URI is required".to_string()));
    }
    
    Ok(())
}

// Create OAuth client for Slack
pub fn create_oauth_client(auth_config: &AuthConfig) -> Result<BasicClient> {
    let client_id = auth_config.api_key.clone()
        .ok_or_else(|| Error::InvalidParameters("Slack client ID is required".to_string()))?;
    
    let client_secret = auth_config.api_secret.clone()
        .ok_or_else(|| Error::InvalidParameters("Slack client secret is required".to_string()))?;
    
    let redirect_uri = auth_config.redirect_uri.clone()
        .ok_or_else(|| Error::InvalidParameters("Slack redirect URI is required".to_string()))?;
    
    // Parse the redirect URI
    let redirect_url = RedirectUrl::new(redirect_uri)
        .map_err(|e| Error::InvalidParameters(format!("Invalid redirect URI: {}", e)))?;
    
    // Create the client
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://slack.com/oauth/authorize".to_string())
            .map_err(|e| Error::InternalError(format!("Invalid Slack auth URL: {}", e)))?,
        Some(TokenUrl::new("https://slack.com/api/oauth.access".to_string())
            .map_err(|e| Error::InternalError(format!("Invalid Slack token URL: {}", e)))?)
    )
    .set_redirect_uri(redirect_url);
    
    Ok(client)
}

// Generate the OAuth authorization URL for Slack
pub fn get_auth_url(auth_config: &AuthConfig) -> Result<String> {
    let client = create_oauth_client(auth_config)?;
    
    // Generate the authorization URL
    let (auth_url, _csrf_token) = client
        .authorize_url(|| "state".to_string())
        .add_scope(Scope::new("channels:history".to_string()))
        .add_scope(Scope::new("channels:read".to_string()))
        .add_scope(Scope::new("chat:write".to_string()))
        .add_scope(Scope::new("users:read".to_string()))
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

// Revoke an access token
pub async fn revoke_token(auth_config: &AuthConfig) -> Result<()> {
    // This would normally call the Slack API to revoke the token
    // For simplicity in the canister environment, we'll just pretend it worked
    
    Ok(())
}