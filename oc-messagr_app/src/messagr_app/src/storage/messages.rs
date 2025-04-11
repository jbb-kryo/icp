use crate::{Message, Error, Result};
use crate::indexing;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    static MESSAGE_STORE: RefCell<StableBTreeMap<String, Message, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );
    
    // Index by conversation_id
    static CONV_MSG_INDEX: RefCell<StableBTreeMap<String, Vec<String>, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );
    
    // Index by timestamp (for range queries)
    static TIME_MSG_INDEX: RefCell<StableBTreeMap<(u64, String), String, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
        )
    );
}

pub fn store_message(message: Message) -> Result<()> {
    let message_id = message.id.clone();
    let conversation_id = message.conversation_id.clone();
    let timestamp = message.timestamp;
    
    // Store the message
    MESSAGE_STORE.with(|store| {
        store.borrow_mut().insert(message_id.clone(), message.clone());
    });
    
    // Update conversation index
    CONV_MSG_INDEX.with(|index| {
        let mut index = index.borrow_mut();
        let mut msg_ids = index.get(&conversation_id).unwrap_or_default();
        if !msg_ids.contains(&message_id) {
            msg_ids.push(message_id.clone());
            index.insert(conversation_id.clone(), msg_ids);
        }
    });
    
    // Update timestamp index
    TIME_MSG_INDEX.with(|index| {
        index.borrow_mut().insert((timestamp, message_id.clone()), message_id.clone());
    });
    
    // Index the message for advanced search
    indexing::index_message(&message)?;
    
    Ok(())
}

pub fn get_message(message_id: &str) -> Option<Message> {
    MESSAGE_STORE.with(|store| {
        store.borrow().get(message_id)
    })
}

pub fn get_conversation_messages(
    conversation_id: &str, 
    limit: usize, 
    before_timestamp: Option<u64>
) -> Vec<Message> {
    // Get message IDs for the conversation
    let message_ids = CONV_MSG_INDEX.with(|index| {
        index.borrow().get(conversation_id).unwrap_or_default()
    });
    
    // Get messages and filter by timestamp if needed
    let mut messages: Vec<Message> = message_ids.iter()
        .filter_map(|id| get_message(id))
        .filter(|msg| {
            if let Some(ts) = before_timestamp {
                msg.timestamp < ts
            } else {
                true
            }
        })
        .collect();
    
    // Sort by timestamp (descending) and apply limit
    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    messages.truncate(limit);
    
    messages
}

pub fn delete_message(message_id: &str) -> Result<()> {
    // Get the message to retrieve its conversation_id and timestamp
    let message = get_message(message_id).ok_or_else(|| {
        Error::InvalidParameters(format!("Message not found: {}", message_id))
    })?;
    
    let conversation_id = message.conversation_id.clone();
    let timestamp = message.timestamp;
    
    // Remove from main store
    MESSAGE_STORE.with(|store| {
        store.borrow_mut().remove(message_id);
    });
    
    // Update conversation index
    CONV_MSG_INDEX.with(|index| {
        let mut index = index.borrow_mut();
        if let Some(mut msg_ids) = index.get(&conversation_id) {
            msg_ids.retain(|id| id != message_id);
            index.insert(conversation_id, msg_ids);
        }
    });
    
    // Remove from timestamp index
    TIME_MSG_INDEX.with(|index| {
        index.borrow_mut().remove(&(timestamp, message_id.to_string()));
    });
    
    // Remove from search indices
    indexing::delete_message(message_id)?;
    
    Ok(())
}

// Get all messages (for rebuilding indices)
pub fn get_all_messages() -> Result<Vec<Message>> {
    MESSAGE_STORE.with(|store| {
        let messages = store.borrow().iter()
            .map(|(_, v)| v.clone())
            .collect();
        
        Ok(messages)
    })
}

// Advanced search using the indexing module
pub fn search_messages(query: &str, filters: &indexing::search::SearchFilters) -> Result<Vec<Message>> {
    // Use the indexing module to perform the search
    let message_ids = indexing::search(query, filters, filters.limit)?;
    
    // Retrieve the actual messages
    let messages: Vec<Message> = message_ids.iter()
        .filter_map(|id| get_message(id))
        .collect();
    
    // Apply sorting based on filters
    let mut sorted_messages = messages;
    
    // Sort by specified field
    match filters.sort_by {
        indexing::search::SortField::Timestamp => {
            sorted_messages.sort_by(|a, b| {
                match filters.sort_direction {
                    indexing::search::SortDirection::Ascending => a.timestamp.cmp(&b.timestamp),
                    indexing::search::SortDirection::Descending => b.timestamp.cmp(&a.timestamp),
                }
            });
        },
        indexing::search::SortField::Platform => {
            sorted_messages.sort_by(|a, b| {
                let platform_cmp = a.platform.to_string().cmp(&b.platform.to_string());
                match filters.sort_direction {
                    indexing::search::SortDirection::Ascending => platform_cmp,
                    indexing::search::SortDirection::Descending => platform_cmp.reverse(),
                }
            });
        },
        indexing::search::SortField::Relevance => {
            // Relevance sorting is already handled by the indexing module
            // but we could add additional relevance factors here
        },
    }
    
    Ok(sorted_messages)
}

// Rebuild all indices
pub fn rebuild_indices() -> Result<()> {
    indexing::reindex_all()?;
    Ok(())
}

// Optimize indices for better performance
pub fn optimize_indices() -> Result<()> {
    indexing::optimize_indices()?;
    Ok(())
}