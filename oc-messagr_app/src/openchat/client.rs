use crate::{Error, Result};
use openchat_sdk::{OpenChatClient, ClientConfig};
use ic_cdk::api::{call, management_canister::main::canister_status, trap};
use ic_cdk::export::candid::{Nat, Principal};
use std::time::Duration;

// Timeout for canister calls in nanoseconds
const CALL_TIMEOUT_NS: u64 = 10_000_000_000; // 10 seconds

// Create an OpenChat SDK client instance
pub async fn create_openchat_client() -> Result<OpenChatClient> {
    // Get the OpenChat canister principal
    let openchat_principal = Principal::from_text(super::OPENCHAT_COMMUNITY_CANISTER_ID)
        .map_err(|e| Error::InternalError(format!("Invalid OpenChat canister ID: {}", e)))?;
    
    // Check if the canister is running
    let status = verify_canister_status(openchat_principal).await?;
    
    // Configure the client
    let config = ClientConfig {
        canister_id: openchat_principal,
        agent_config: None, // Use default agent configuration
        query_timeout: Some(Duration::from_secs(30)),
        update_timeout: Some(Duration::from_secs(60)),
    };
    
    // Create the client
    let client = OpenChatClient::new(config)
        .map_err(|e| Error::InternalError(format!("Failed to create OpenChat client: {}", e)))?;
    
    Ok(client)
}

// Verify that the OpenChat canister is running
async fn verify_canister_status(canister_id: Principal) -> Result<()> {
    let arg = ic_cdk::export::candid::encode_one(canister_id)
        .map_err(|e| Error::InternalError(format!("Failed to encode canister ID: {}", e)))?;
    
    let (result,): (canister_status::CanisterStatusResponse,) = 
        call::call_with_payment(
            Principal::management_canister(), 
            "canister_status", 
            (arg,), 
            CALL_TIMEOUT_NS
        )
        .await
        .map_err(|e| Error::InternalError(format!("Failed to call canister_status: {:?}", e)))?;
    
    // Check if the canister is running
    if result.status != canister_status::Status::Running {
        return Err(Error::InternalError(format!(
            "OpenChat canister is not running. Current status: {:?}", 
            result.status
        )));
    }
    
    Ok(())
}

// Get cycles balance of the current canister
pub async fn get_cycles_balance() -> Result<u64> {
    let self_id = ic_cdk::id();
    
    let arg = ic_cdk::export::candid::encode_one(self_id)
        .map_err(|e| Error::InternalError(format!("Failed to encode canister ID: {}", e)))?;
    
    let (result,): (canister_status::CanisterStatusResponse,) = 
        call::call_with_payment(
            Principal::management_canister(), 
            "canister_status", 
            (arg,), 
            CALL_TIMEOUT_NS
        )
        .await
        .map_err(|e| Error::InternalError(format!("Failed to call canister_status: {:?}", e)))?;
    
    // Convert cycles balance from Nat to u64
    let cycles: u64 = result.cycles.0.try_into()
        .map_err(|e| Error::InternalError(format!("Failed to convert cycles to u64: {}", e)))?;
    
    Ok(cycles)
}