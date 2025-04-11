use crate::{Message, Error, Result};
use tantivy::{Index, IndexWriter, Document, Term, query::QueryParser, collector::TopDocs};
use tantivy::query::{Query, BooleanQuery, Occur, TermQuery, RangeQuery};
use tantivy::schema::{Field, Value};
use tantivy::directory::MmapDirectory;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use super::schema::{create_metadata_schema, platform_to_string, FIELD_ID, FIELD_CONVERSATION_ID, FIELD_PLATFORM, FIELD_TIMESTAMP};
use super::search::SearchFilters;

// In-memory buffer size
const MEMORY_BUFFER_SIZE: usize = 20_000_000; // 20MB

// Path for metadata index storage
const INDEX_PATH: &str = "stable_memory/metadata_index";

// Flag for operating in test mode
static IN_TEST_MODE: AtomicBool = AtomicBool::new(false);

pub struct MetadataIndexer {
    index: Index,
    writer: IndexWriter,
    fields: HashMap<String, Field>,
}

impl MetadataIndexer {
    pub fn new() -> Self {
        // Set up the schema
        let (schema, fields) = create_metadata_schema();
        
        // Create or open the index
        let index = if IN_TEST_MODE.load(Ordering::Relaxed) {
            // In-memory index for testing
            Index::create_in_ram(schema)
        } else {
            // Create directory if it doesn't exist
            if !Path::new(INDEX_PATH).exists() {
                fs::create_dir_all(INDEX_PATH).unwrap_or_else(|e| {
                    ic_cdk::println!("Error creating metadata index directory: {}", e);
                });
            }
            
            // Persistent index
            let dir = MmapDirectory::open(Path::new(INDEX_PATH))
                .unwrap_or_else(|e| {
                    ic_cdk::println!("Error opening metadata index directory: {}", e);
                    // Fallback to in-memory
                    MmapDirectory::create_from_tempdir().unwrap()
                });
            
            Index::open_or_create(dir, schema).unwrap_or_else(|e| {
                ic_cdk::println!("Error opening metadata index: {}", e);
                // Fallback to in-memory
                Index::create_in_ram(schema)
            })
        };
        
        // Create a writer with a memory buffer
        let writer = index.writer(MEMORY_BUFFER_SIZE)
            .unwrap_or_else(|e| {
                ic_cdk::println!("Error creating metadata index writer: {}", e);
                // Fallback with smaller buffer
                index.writer(1_000_000).unwrap()
            });
        
        Self {
            index,
            writer,
            fields,
        }
    }
    
    // Index a message's metadata
    pub fn index_message(&mut self, message: &Message) -> Result<()> {
        let mut doc = Document::new();
        
        // Add core fields
        doc.add_text(self.fields[FIELD_ID], &message.id);
        doc.add_text(self.fields[FIELD_CONVERSATION_ID], &message.conversation_id);
        doc.add_text(self.fields[FIELD_PLATFORM], &platform_to_string(&message.platform));
        doc.add_u64(self.fields[FIELD_TIMESTAMP], message.timestamp);
        
        // Add sender ID
        doc.add_text(self.fields["sender_id"], &message.sender.id);
        
        // Has attachments flag
        doc.add_bool(self.fields["has_attachments"], !message.content.attachments.is_empty());
        
        // Is edited flag
        doc.add_bool(self.fields["is_edited"], message.edited);
        
        // Add the document to the index
        self.writer.add_document(doc).map_err(|e| Error::InternalError(format!("Failed to index message metadata: {}", e)))?;
        
        // Commit periodically
        if message.id.ends_with("00") {
            self.commit()?;
        }
        
        Ok(())
    }
    
    // Commit changes to the index
    fn commit(&mut self) -> Result<()> {
        self.writer.commit().map_err(|e| Error::InternalError(format!("Failed to commit metadata index: {}", e)))?;
        Ok(())
    }
    
    // Optimize the index
    pub fn optimize(&mut self) -> Result<()> {
        // First commit any pending changes
        self.commit()?;
        
        // Then merge segments
        self.writer.merge(&[]).map_err(|e| Error::InternalError(format!("Failed to optimize metadata index: {}", e)))?;
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
        self.writer.delete_all_documents().map_err(|e| Error::InternalError(format!("Failed to clear metadata index: {}", e)))?;
        self.commit()?;
        Ok(())
    }
    
    // Filter messages based on metadata
    pub fn filter(&self, filters: &SearchFilters, limit: usize) -> Result<HashSet<String>> {
        // Create a boolean query from filters
        let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();
        
        // Platform filter
        if let Some(platform) = &filters.platform {
            let platform_term = Term::from_field_text(
                self.fields[FIELD_PLATFORM],
                &platform_to_string(platform)
            );
            clauses.push((Occur::Must, Box::new(TermQuery::new(platform_term, 1.0))));
        }
        
        // Time range filters
        if let Some(start_time) = filters.start_time {
            clauses.push((
                Occur::Must,
                Box::new(RangeQuery::new_u64(
                    self.fields[FIELD_TIMESTAMP],
                    start_time..=u64::MAX
                ))
            ));
        }
        
        if let Some(end_time) = filters.end_time {
            clauses.push((
                Occur::Must,
                Box::new(RangeQuery::new_u64(
                    self.fields[FIELD_TIMESTAMP],
                    0..=end_time
                ))
            ));
        }
        
        // Conversation ID filter
        if let Some(conv_id) = &filters.conversation_id {
            let conv_term = Term::from_field_text(
                self.fields[FIELD_CONVERSATION_ID],
                conv_id
            );
            clauses.push((Occur::Must, Box::new(TermQuery::new(conv_term, 1.0))));
        }
        
        // Sender ID filter
        if let Some(sender_id) = &filters.sender_id {
            let sender_term = Term::from_field_text(
                self.fields["sender_id"],
                sender_id
            );
            clauses.push((Occur::Must, Box::new(TermQuery::new(sender_term, 1.0))));
        }
        
        // Has attachments filter
        if filters.has_attachments {
            let attachment_term = Term::from_field_bool(
                self.fields["has_attachments"],
                true
            );
            clauses.push((Occur::Must, Box::new(TermQuery::new(attachment_term, 1.0))));
        }
        
        // Is edited filter
        if filters.is_edited {
            let edited_term = Term::from_field_bool(
                self.fields["is_edited"],
                true
            );
            clauses.push((Occur::Must, Box::new(TermQuery::new(edited_term, 1.0))));
        }
        
        // Create the final query
        let boolean_query = if clauses.is_empty() {
            // Match all documents if no filters are specified
            let query_parser = QueryParser::for_index(&self.index, vec![]);
            query_parser.parse_query("*").unwrap()
        } else {
            Box::new(BooleanQuery::new(clauses))
        };
        
        // Execute the search
        let searcher = self.index.reader()
            .map_err(|e| Error::InternalError(format!("Failed to get metadata index reader: {}", e)))?
            .searcher();
        
        let top_docs = searcher.search(&boolean_query, &TopDocs::with_limit(limit))
            .map_err(|e| Error::InternalError(format!("Failed to execute metadata search: {}", e)))?;
        
        // Extract message IDs
        let mut results = HashSet::new();
        
        for (_, doc_address) in top_docs {
            // Retrieve the document
            let retrieved_doc = searcher.doc(doc_address)
                .map_err(|e| Error::InternalError(format!("Failed to retrieve metadata document: {}", e)))?;
            
            // Extract the message ID
            let id_field = self.fields[FIELD_ID];
            if let Some(id_value) = retrieved_doc.get_first(id_field) {
                if let Some(id_text) = id_value.as_text() {
                    results.insert(id_text.to_string());
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