pub mod schema;
pub mod search;
pub mod text;
pub mod metadata;
pub mod attachments;

use crate::{Message, Conversation, Error, Result, Platform};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use ic_cdk::api::time;
use std::collections::{HashSet, HashMap};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// IndexManager is responsible for coordinating all indexing operations
pub struct IndexManager {
    text_indexer: text::TextIndexer,
    metadata_indexer: metadata::MetadataIndexer,
    attachment_indexer: attachments::AttachmentIndexer,
}

impl IndexManager {
    pub fn new() -> Self {
        Self {
            text_indexer: text::TextIndexer::new(),
            metadata_indexer: metadata::MetadataIndexer::new(),
            attachment_indexer: attachments::AttachmentIndexer::new(),
        }
    }

    // Index a new message
    pub fn index_message(&mut self, message: &Message) -> Result<()> {
        // Index text content
        self.text_indexer.index_message(message)?;
        
        // Index metadata (sender, timestamp, platform, etc.)
        self.metadata_indexer.index_message(message)?;
        
        // Index attachments
        if !message.content.attachments.is_empty() {
            self.attachment_indexer.index_message(message)?;
        }
        
        Ok(())
    }
    
    // Reindex all messages (for example after schema changes)
    pub fn reindex_all_messages(&mut self, messages: &[Message]) -> Result<()> {
        // Clear existing indices
        self.text_indexer.clear()?;
        self.metadata_indexer.clear()?;
        self.attachment_indexer.clear()?;
        
        // Reindex all messages
        for message in messages {
            self.index_message(message)?;
        }
        
        Ok(())
    }
    
    // Search across all indices
    pub fn search(&self, query: &str, filters: &search::SearchFilters, limit: usize) -> Result<Vec<String>> {
        // Collect results from each indexer
        let text_results = self.text_indexer.search(query, limit * 2)?;
        let metadata_results = self.metadata_indexer.filter(&filters, limit * 2)?;
        let attachment_results = if filters.has_attachments {
            self.attachment_indexer.search(query, filters, limit * 2)?
        } else {
            HashSet::new()
        };
        
        // Combine and rank results
        let combined_results = self.rank_results(text_results, metadata_results, attachment_results, query);
        
        // Apply limit
        let limited_results: Vec<String> = combined_results
            .into_iter()
            .take(limit)
            .collect();
        
        Ok(limited_results)
    }
    
    // Rank and combine results from different indices
    fn rank_results(
        &self, 
        text_results: HashMap<String, f32>, 
        metadata_results: HashSet<String>,
        attachment_results: HashSet<String>,
        query: &str
    ) -> Vec<String> {
        // Create a score map for all results
        let mut scores: HashMap<String, f32> = HashMap::new();
        
        // Add text scores (already normalized 0.0-1.0)
        for (id, score) in text_results {
            scores.insert(id, score);
        }
        
        // Add metadata bonus (0.2)
        for id in metadata_results {
            *scores.entry(id).or_insert(0.0) += 0.2;
        }
        
        // Add attachment bonus (0.1)
        for id in attachment_results {
            *scores.entry(id).or_insert(0.0) += 0.1;
        }
        
        // Convert to vec and sort by score
        let mut ranked_results: Vec<(String, f32)> = scores.into_iter().collect();
        ranked_results.sort_by(|(_, score_a), (_, score_b)| score_b.partial_cmp(score_a).unwrap());
        
        // Return just the message IDs
        ranked_results.into_iter().map(|(id, _)| id).collect()
    }
    
    // Optimize indices for better performance
    pub fn optimize(&mut self) -> Result<()> {
        self.text_indexer.optimize()?;
        self.metadata_indexer.optimize()?;
        self.attachment_indexer.optimize()?;
        Ok(())
    }
    
    // Delete a message from all indices
    pub fn delete_message(&mut self, message_id: &str) -> Result<()> {
        self.text_indexer.delete_message(message_id)?;
        self.metadata_indexer.delete_message(message_id)?;
        self.attachment_indexer.delete_message(message_id)?;
        Ok(())
    }
}

// Global index management
thread_local! {
    static INDEX_MANAGER: RefCell<IndexManager> = RefCell::new(IndexManager::new());
}

// Public API functions
pub fn index_message(message: &Message) -> Result<()> {
    INDEX_MANAGER.with(|manager| {
        manager.borrow_mut().index_message(message)
    })
}

pub fn search(query: &str, filters: &search::SearchFilters, limit: usize) -> Result<Vec<String>> {
    INDEX_MANAGER.with(|manager| {
        manager.borrow().search(query, filters, limit)
    })
}

pub fn delete_message(message_id: &str) -> Result<()> {
    INDEX_MANAGER.with(|manager| {
        manager.borrow_mut().delete_message(message_id)
    })
}

pub fn optimize_indices() -> Result<()> {
    INDEX_MANAGER.with(|manager| {
        manager.borrow_mut().optimize()
    })
}

pub fn reindex_all() -> Result<()> {
    // Get all messages from storage
    crate::storage::messages::get_all_messages().and_then(|messages| {
        INDEX_MANAGER.with(|manager| {
            manager.borrow_mut().reindex_all_messages(&messages)
        })
    })
}