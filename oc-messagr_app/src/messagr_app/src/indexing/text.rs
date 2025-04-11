use crate::{Message, Error, Result};
use tantivy::{Index, IndexWriter, Document, Term, query::QueryParser, collector::TopDocs, TantivyError};
use tantivy::schema::Field;
use tantivy::directory::MmapDirectory;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use super::schema::{create_text_schema, register_tokenizers, platform_to_string, FIELD_ID, FIELD_CONTENT, FIELD_SENDER_NAME, FIELD_CONVERSATION_ID, FIELD_PLATFORM, FIELD_TIMESTAMP, FIELD_THREAD_ID, FIELD_REPLY_TO};

// In-memory buffer before writing to stable storage
const MEMORY_BUFFER_SIZE: usize = 50_000_000; // 50MB

// Path for index storage (would need to be adapted for canister stable memory)
const INDEX_PATH: &str = "stable_memory/text_index";

// Flag for operating in test/production mode
static IN_TEST_MODE: AtomicBool = AtomicBool::new(false);

pub struct TextIndexer {
    index: Index,
    writer: IndexWriter,
    fields: HashMap<String, Field>,
}

impl TextIndexer {
    pub fn new() -> Self {
        // Set up the schema
        let (schema, fields) = create_text_schema();
        
        // Register custom tokenizers
        register_tokenizers();
        
        // Create or open the index
        let index = if IN_TEST_MODE.load(Ordering::Relaxed) {
            // In-memory index for testing
            Index::create_in_ram(schema)
        } else {
            // Create directory if it doesn't exist
            if !Path::new(INDEX_PATH).exists() {
                fs::create_dir_all(INDEX_PATH).unwrap_or_else(|e| {
                    ic_cdk::println!("Error creating index directory: {}", e);
                });
            }
            
            // Persistent index
            let dir = MmapDirectory::open(Path::new(INDEX_PATH))
                .unwrap_or_else(|e| {
                    ic_cdk::println!("Error opening index directory: {}", e);
                    // Fallback to in-memory
                    MmapDirectory::create_from_tempdir().unwrap()
                });
            
            Index::open_or_create(dir, schema).unwrap_or_else(|e| {
                ic_cdk::println!("Error opening index: {}", e);
                // Fallback to in-memory
                Index::create_in_ram(schema)
            })
        };
        
        // Create a writer with a memory buffer
        let writer = index.writer(MEMORY_BUFFER_SIZE)
            .unwrap_or_else(|e| {
                ic_cdk::println!("Error creating index writer: {}", e);
                // Fallback with smaller buffer
                index.writer(1_000_000).unwrap()
            });
        
        Self {
            index,
            writer,
            fields,
        }
    }
    
    // Index a message
    pub fn index_message(&mut self, message: &Message) -> Result<()> {
        let mut doc = Document::new();
        
        // Add all fields to the document
        doc.add_text(self.fields[FIELD_ID], &message.id);
        doc.add_text(self.fields[FIELD_CONTENT], &message.content.text);
        doc.add_text(self.fields[FIELD_SENDER_NAME], &message.sender.name);
        doc.add_text(self.fields[FIELD_CONVERSATION_ID], &message.conversation_id);
        doc.add_text(self.fields[FIELD_PLATFORM], &platform_to_string(&message.platform));
        doc.add_u64(self.fields[FIELD_TIMESTAMP], message.timestamp);
        
        // Add optional fields if available
        if let Some(thread_id) = &message.thread_id {
            doc.add_text(self.fields[FIELD_THREAD_ID], thread_id);
        }
        
        if let Some(reply_to) = &message.reply_to {
            doc.add_text(self.fields[FIELD_REPLY_TO], reply_to);
        }
        
        // Add the document to the index
        self.writer.add_document(doc).map_err(|e| Error::InternalError(format!("Failed to index message: {}", e)))?;
        
        // Commit periodically to avoid excessive memory usage
        // In a real implementation, we would use a more sophisticated commit strategy
        if message.id.ends_with("00") { // Arbitrary condition for demo
            self.commit()?;
        }
        
        Ok(())
    }
    
    // Commit changes to the index
    fn commit(&mut self) -> Result<()> {
        self.writer.commit().map_err(|e| Error::InternalError(format!("Failed to commit index: {}", e)))?;
        Ok(())
    }
    
    // Optimize the index for better performance
    pub fn optimize(&mut self) -> Result<()> {
        // First commit any pending changes
        self.commit()?;
        
        // Then merge segments for better performance
        self.writer.merge(&[]).map_err(|e| Error::InternalError(format!("Failed to optimize index: {}", e)))?;
        Ok(())
    }
    
    // Delete a message from the index
    pub fn delete_message(&mut self, message_id: &str) -> Result<()> {
        let term = Term::from_field_text(self.fields[FIELD_ID], message_id);
        self.writer.delete_term(term);
        self.commit()?;
        Ok(())
    }
    
    // Clear the entire index
    pub fn clear(&mut self) -> Result<()> {
        self.writer.delete_all_documents().map_err(|e| Error::InternalError(format!("Failed to clear index: {}", e)))?;
        self.commit()?;
        Ok(())
    }
    
    // Search for messages
    pub fn search(&self, query_text: &str, limit: usize) -> Result<HashMap<String, f32>> {
        // Create a query parser that searches in multiple fields with different boosts
        let mut query_parser = QueryParser::for_index(
            &self.index, 
            vec![
                (self.fields[FIELD_CONTENT], 1.0),      // Normal boost for content
                (self.fields[FIELD_SENDER_NAME], 0.5),  // Lower boost for sender
            ]
        );
        
        // Set default conjunctive operator (AND)
        query_parser.set_conjunction_by_default();
        
        // Parse the query
        let query = query_parser.parse_query(query_text)
            .map_err(|e| Error::QueryError(format!("Failed to parse query: {}", e)))?;
        
        // Execute the search
        let searcher = self.index.reader()
            .map_err(|e| Error::InternalError(format!("Failed to get index reader: {}", e)))?
            .searcher();
        
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))
            .map_err(|e| Error::InternalError(format!("Failed to execute search: {}", e)))?;
        
        // Convert results to ID -> score map
        let mut results = HashMap::new();
        
        // Find the maximum score for normalization
        let max_score = top_docs.iter()
            .map(|(score, _)| score)
            .fold(0.0, |max, &score| if score > max { score } else { max });
        
        for (score, doc_address) in top_docs {
            // Normalize score to 0.0-1.0 range
            let normalized_score = if max_score > 0.0 { score / max_score } else { 0.0 };
            
            // Retrieve the document
            let retrieved_doc = searcher.doc(doc_address)
                .map_err(|e| Error::InternalError(format!("Failed to retrieve document: {}", e)))?;
            
            // Extract the message ID
            let id_field = self.fields[FIELD_ID];
            if let Some(id_value) = retrieved_doc.get_first(id_field) {
                if let Some(id_text) = id_value.as_text() {
                    results.insert(id_text.to_string(), normalized_score);
                }
            }
        }
        
        Ok(results)
    }
}

// For testing
pub fn set_test_mode(enabled: bool) {
    IN_TEST_MODE.store(enabled, Ordering::Relaxed);
}