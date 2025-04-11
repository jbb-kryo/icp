use tantivy::schema::{Schema, STORED, INDEXED, TEXT, STRING, FAST, SchemaBuilder, Field, TextOptions, TextFieldIndexing};
use tantivy::tokenizer::{NgramTokenizer, TextAnalyzer, SimpleTokenizer, Language, LowerCaser, StopWordFilter, Stemmer, RemoveLongFilter};
use std::collections::HashMap;
use crate::Platform;

// Field names for text index
pub const FIELD_ID: &str = "id";
pub const FIELD_CONTENT: &str = "content";
pub const FIELD_SENDER_NAME: &str = "sender_name";
pub const FIELD_CONVERSATION_ID: &str = "conversation_id";
pub const FIELD_PLATFORM: &str = "platform";
pub const FIELD_TIMESTAMP: &str = "timestamp";
pub const FIELD_THREAD_ID: &str = "thread_id";
pub const FIELD_REPLY_TO: &str = "reply_to";

// Attachment field names
pub const FIELD_ATTACHMENT_TYPE: &str = "attachment_type";
pub const FIELD_ATTACHMENT_NAME: &str = "attachment_name";
pub const FIELD_ATTACHMENT_URL: &str = "attachment_url";

// Create the text schema for message content
pub fn create_text_schema() -> (Schema, HashMap<String, Field>) {
    let mut schema_builder = Schema::builder();
    let mut fields = HashMap::new();
    
    // Configure text field indexing with advanced analysis
    let text_indexing = TextFieldIndexing::default()
        .set_tokenizer("en_ngram") // Custom tokenizer
        .set_index_option(tantivy::schema::IndexRecordOption::WithFreqsAndPositions);
    
    let text_options = TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored();
    
    // ID field - stored but not tokenized
    fields.insert(
        FIELD_ID.to_string(),
        schema_builder.add_text_field(FIELD_ID, STRING | STORED)
    );
    
    // Content field - fully indexed and analyzed
    fields.insert(
        FIELD_CONTENT.to_string(),
        schema_builder.add_text_field(FIELD_CONTENT, text_options)
    );
    
    // Sender name - indexed and stored
    fields.insert(
        FIELD_SENDER_NAME.to_string(),
        schema_builder.add_text_field(FIELD_SENDER_NAME, TEXT | STORED)
    );
    
    // Conversation ID - stored but not tokenized
    fields.insert(
        FIELD_CONVERSATION_ID.to_string(),
        schema_builder.add_text_field(FIELD_CONVERSATION_ID, STRING | STORED)
    );
    
    // Platform - stored as string
    fields.insert(
        FIELD_PLATFORM.to_string(),
        schema_builder.add_text_field(FIELD_PLATFORM, STRING | STORED)
    );
    
    // Timestamp - stored as u64
    fields.insert(
        FIELD_TIMESTAMP.to_string(),
        schema_builder.add_u64_field(FIELD_TIMESTAMP, STORED | INDEXED | FAST)
    );
    
    // Thread ID - optional, stored as string
    fields.insert(
        FIELD_THREAD_ID.to_string(),
        schema_builder.add_text_field(FIELD_THREAD_ID, STRING | STORED)
    );
    
    // Reply to - optional, stored as string
    fields.insert(
        FIELD_REPLY_TO.to_string(),
        schema_builder.add_text_field(FIELD_REPLY_TO, STRING | STORED)
    );
    
    let schema = schema_builder.build();
    (schema, fields)
}

// Create schema for attachment indexing
pub fn create_attachment_schema() -> (Schema, HashMap<String, Field>) {
    let mut schema_builder = Schema::builder();
    let mut fields = HashMap::new();
    
    // Message ID - stored but not tokenized
    fields.insert(
        FIELD_ID.to_string(),
        schema_builder.add_text_field(FIELD_ID, STRING | STORED)
    );
    
    // Attachment type
    fields.insert(
        FIELD_ATTACHMENT_TYPE.to_string(),
        schema_builder.add_text_field(FIELD_ATTACHMENT_TYPE, STRING | STORED)
    );
    
    // Attachment name - indexed and stored
    let text_options = TextOptions::default()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("standard")
                .set_index_option(tantivy::schema::IndexRecordOption::WithFreqsAndPositions)
        )
        .set_stored();
    
    fields.insert(
        FIELD_ATTACHMENT_NAME.to_string(),
        schema_builder.add_text_field(FIELD_ATTACHMENT_NAME, text_options)
    );
    
    // Attachment URL - stored but not tokenized
    fields.insert(
        FIELD_ATTACHMENT_URL.to_string(),
        schema_builder.add_text_field(FIELD_ATTACHMENT_URL, STRING | STORED)
    );
    
    // Add common fields
    fields.insert(
        FIELD_TIMESTAMP.to_string(),
        schema_builder.add_u64_field(FIELD_TIMESTAMP, STORED | INDEXED | FAST)
    );
    
    fields.insert(
        FIELD_PLATFORM.to_string(),
        schema_builder.add_text_field(FIELD_PLATFORM, STRING | STORED)
    );
    
    fields.insert(
        FIELD_CONVERSATION_ID.to_string(),
        schema_builder.add_text_field(FIELD_CONVERSATION_ID, STRING | STORED)
    );
    
    let schema = schema_builder.build();
    (schema, fields)
}

// Create schema for metadata indexing
pub fn create_metadata_schema() -> (Schema, HashMap<String, Field>) {
    let mut schema_builder = Schema::builder();
    let mut fields = HashMap::new();
    
    // Message ID - stored but not tokenized
    fields.insert(
        FIELD_ID.to_string(),
        schema_builder.add_text_field(FIELD_ID, STRING | STORED)
    );
    
    // Timestamp - stored as u64 with fast field options for range filtering
    fields.insert(
        FIELD_TIMESTAMP.to_string(),
        schema_builder.add_u64_field(FIELD_TIMESTAMP, STORED | INDEXED | FAST)
    );
    
    // Platform - stored as string
    fields.insert(
        FIELD_PLATFORM.to_string(),
        schema_builder.add_text_field(FIELD_PLATFORM, STRING | STORED)
    );
    
    // Conversation ID - stored but not tokenized
    fields.insert(
        FIELD_CONVERSATION_ID.to_string(),
        schema_builder.add_text_field(FIELD_CONVERSATION_ID, STRING | STORED)
    );
    
    // Sender ID - stored but not tokenized
    fields.insert(
        "sender_id".to_string(),
        schema_builder.add_text_field("sender_id", STRING | STORED)
    );
    
    // Has attachments flag
    fields.insert(
        "has_attachments".to_string(),
        schema_builder.add_bool_field("has_attachments", STORED | INDEXED)
    );
    
    // Is edited flag
    fields.insert(
        "is_edited".to_string(),
        schema_builder.add_bool_field("is_edited", STORED | INDEXED)
    );
    
    let schema = schema_builder.build();
    (schema, fields)
}

// Register custom tokenizers
pub fn register_tokenizers() {
    // English analyzer with stemming and stopwords
    let en_analyzer = TextAnalyzer::from(SimpleTokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(StopWordFilter::new(Language::English))
        .filter(Stemmer::new(Language::English));
    
    tantivy::tokenizer::TokenizerManager::register("en_analyzer", en_analyzer);
    
    // N-gram tokenizer for partial matching and typo tolerance
    let ngram = TextAnalyzer::from(SimpleTokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(NgramTokenizer::new(2, 4, false).unwrap());
    
    tantivy::tokenizer::TokenizerManager::register("en_ngram", ngram);
}

// Convert Platform enum to string for storage
pub fn platform_to_string(platform: &Platform) -> String {
    match platform {
        Platform::Telegram => "telegram".to_string(),
        Platform::Slack => "slack".to_string(),
        Platform::Discord => "discord".to_string(),
        Platform::Twitter => "twitter".to_string(),
        Platform::Facebook => "facebook".to_string(),
        Platform::WhatsApp => "whatsapp".to_string(),
    }
}