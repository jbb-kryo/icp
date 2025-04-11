use crate::{Conversation, Platform, User, Error, Result};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use candid::Principal;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    static CONV_STORE: RefCell<StableBTreeMap<String, Conversation, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
    
    // Index by user ID to quickly find conversations for a user
    static USER_CONV_INDEX: RefCell<StableBTreeMap<String, Vec<String>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))),
        )
    );
    
    // Index by platform
    static PLATFORM_CONV_INDEX: RefCell<StableBTreeMap<String, Vec<String>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))),
        )
    );
}

pub fn store_conversation(conversation: Conversation) -> Result<()> {
    let conversation_id = conversation.id.clone();
    let platform_str = platform_to_string(&conversation.platform);
    
    // Store the conversation
    CONV_STORE.with(|store| {
        store.borrow_mut().insert(conversation_id.clone(), conversation.clone());
    });
    
    // Update user index for each participant
    for participant in &conversation.participants {
        USER_CONV_INDEX.with(|index| {
            let mut index = index.borrow_mut();
            let mut conv_ids = index.get(&participant.id).unwrap_or_default();
            if !conv_ids.contains(&conversation_id) {
                conv_ids.push(conversation_id.clone());
                index.insert(participant.id.clone(), conv_ids);
            }
        });
    }
    
    // Update platform index
    PLATFORM_CONV_INDEX.with(|index| {
        let mut index = index.borrow_mut();
        let mut conv_ids = index.get(&platform_str).unwrap_or_default();
        if !conv_ids.contains(&conversation_id) {
            conv_ids.push(conversation_id.clone());
            index.insert(platform_str.clone(), conv_ids);
        }
    });
    
    Ok(())
}

pub fn get_conversation(conversation_id: &str) -> Option<Conversation> {
    CONV_STORE.with(|store| {
        store.borrow().get(conversation_id)
    })
}

pub fn get_user_conversations(user_id: &str, platform: Option<Platform>) -> Vec<Conversation> {
    // Get conversation IDs for the user
    let conversation_ids = USER_CONV_INDEX.with(|index| {
        index.borrow().get(user_id).unwrap_or_default()
    });
    
    // Filter by platform if specified
    let conversations: Vec<Conversation> = conversation_ids.iter()
        .filter_map(|id| get_conversation(id))
        .filter(|conv| {
            if let Some(p) = &platform {
                platform_to_string(p) == platform_to_string(&conv.platform)
            } else {
                true
            }
        })
        .collect();
    
    conversations
}

pub fn delete_conversation(conversation_id: &str) -> Result<()> {
    // Get the conversation to retrieve its platform and participants
    let conversation = get_conversation(conversation_id).ok_or_else(|| {
        Error::InvalidParameters(format!("Conversation not found: {}", conversation_id))
    })?;
    
    let platform_str = platform_to_string(&conversation.platform);
    
    // Remove from main store
    CONV_STORE.with(|store| {
        store.borrow_mut().remove(conversation_id);
    });
    
    // Update user indices
    for participant in &conversation.participants {
        USER_CONV_INDEX.with(|index| {
            let mut index = index.borrow_mut();
            if let Some(mut conv_ids) = index.get(&participant.id) {
                conv_ids.retain(|id| id != conversation_id);
                index.insert(participant.id.clone(), conv_ids);
            }
        });
    }
    
    // Update platform index
    PLATFORM_CONV_INDEX.with(|index| {
        let mut index = index.borrow_mut();
        if let Some(mut conv_ids) = index.get(&platform_str) {
            conv_ids.retain(|id| id != conversation_id);
            index.insert(platform_str, conv_ids);
        }
    });
    
    Ok(())
}

pub fn update_conversation_last_message(conversation_id: &str, timestamp: u64) -> Result<()> {
    let conversation = get_conversation(conversation_id).ok_or_else(|| {
        Error::InvalidParameters(format!("Conversation not found: {}", conversation_id))
    })?;
    
    let mut updated = conversation.clone();
    updated.last_message_at = Some(timestamp);
    
    CONV_STORE.with(|store| {
        store.borrow_mut().insert(conversation_id.to_string(), updated);
    });
    
    Ok(())
}

fn platform_to_string(platform: &Platform) -> String {
    match platform {
        Platform::Telegram => "telegram".to_string(),
        Platform::Slack => "slack".to_string(),
        Platform::Discord => "discord".to_string(),
        Platform::Twitter => "twitter".to_string(),
        Platform::Facebook => "facebook".to_string(),
        Platform::WhatsApp => "whatsapp".to_string(),
    }
}