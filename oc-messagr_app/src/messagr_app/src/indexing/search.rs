use crate::Platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    // Platform specific filters
    pub platform: Option<Platform>,
    
    // Time range filters
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    
    // Conversation filters
    pub conversation_id: Option<String>,
    
    // Sender filters
    pub sender_id: Option<String>,
    pub sender_name: Option<String>,
    
    // Content type filters
    pub has_attachments: bool,
    pub attachment_type: Option<String>,
    
    // Message type filters
    pub is_reply: bool,
    pub in_thread: bool,
    pub is_edited: bool,
    
    // Sort options
    pub sort_by: SortField,
    pub sort_direction: SortDirection,
    
    // Pagination
    pub offset: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortField {
    Relevance,
    Timestamp,
    Platform,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            platform: None,
            start_time: None,
            end_time: None,
            conversation_id: None,
            sender_id: None,
            sender_name: None,
            has_attachments: false,
            attachment_type: None,
            is_reply: false,
            in_thread: false,
            is_edited: false,
            sort_by: SortField::Relevance,
            sort_direction: SortDirection::Descending,
            offset: 0,
            limit: 50,
        }
    }
}

impl SearchFilters {
    // Builder pattern methods for easier construction
    pub fn with_platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }
    
    pub fn with_time_range(mut self, start: u64, end: u64) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }
    
    pub fn with_conversation(mut self, conversation_id: String) -> Self {
        self.conversation_id = Some(conversation_id);
        self
    }
    
    pub fn with_sender_id(mut self, sender_id: String) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
    
    pub fn with_sender_name(mut self, sender_name: String) -> Self {
        self.sender_name = Some(sender_name);
        self
    }
    
    pub fn with_attachments(mut self, has_attachments: bool) -> Self {
        self.has_attachments = has_attachments;
        self
    }
    
    pub fn with_attachment_type(mut self, attachment_type: String) -> Self {
        self.attachment_type = Some(attachment_type);
        self.has_attachments = true;
        self
    }
    
    pub fn with_replies_only(mut self, is_reply: bool) -> Self {
        self.is_reply = is_reply;
        self
    }
    
    pub fn with_threads_only(mut self, in_thread: bool) -> Self {
        self.in_thread = in_thread;
        self
    }
    
    pub fn with_edited_only(mut self, is_edited: bool) -> Self {
        self.is_edited = is_edited;
        self
    }
    
    pub fn sort_by(mut self, field: SortField, direction: SortDirection) -> Self {
        self.sort_by = field;
        self.sort_direction = direction;
        self
    }
    
    pub fn with_pagination(mut self, offset: usize, limit: usize) -> Self {
        self.offset = offset;
        self.limit = limit;
        self
    }
    
    // Parse a natural language query into structured filters
    pub fn from_natural_language(query: &str) -> (String, Self) {
        let mut filters = SearchFilters::default();
        let mut cleaned_query = String::new();
        
        let words: Vec<&str> = query.split_whitespace().collect();
        let mut i = 0;
        
        while i < words.len() {
            let word = words[i].to_lowercase();
            
            // Platform filters
            if word == "telegram" || word == "from:telegram" {
                filters.platform = Some(Platform::Telegram);
            } else if word == "slack" || word == "from:slack" {
                filters.platform = Some(Platform::Slack);
            } else if word == "discord" || word == "from:discord" {
                filters.platform = Some(Platform::Discord);
            } else if word == "twitter" || word == "from:twitter" {
                filters.platform = Some(Platform::Twitter);
            } else if word == "facebook" || word == "from:facebook" {
                filters.platform = Some(Platform::Facebook);
            } else if word == "whatsapp" || word == "from:whatsapp" {
                filters.platform = Some(Platform::WhatsApp);
            }
            // Time-based filters
            else if word == "today" {
                let now = ic_cdk::api::time() / 1_000_000;
                let day_start = now - (now % 86400);
                filters.start_time = Some(day_start * 1000);
                filters.end_time = Some(now * 1000);
            } else if word == "yesterday" {
                let now = ic_cdk::api::time() / 1_000_000;
                let day_start = now - (now % 86400);
                let yesterday_start = day_start - 86400;
                let yesterday_end = day_start - 1;
                filters.start_time = Some(yesterday_start * 1000);
                filters.end_time = Some(yesterday_end * 1000);
            } else if word == "this" && i + 1 < words.len() && words[i + 1] == "week" {
                let now = ic_cdk::api::time() / 1_000_000;
                let week_start = now - ((now % 86400) + 86400 * (now / 86400) % 7);
                filters.start_time = Some(week_start * 1000);
                filters.end_time = Some(now * 1000);
                i += 1; // Skip the next word
            } else if word == "last" && i + 1 < words.len() && words[i + 1] == "week" {
                let now = ic_cdk::api::time() / 1_000_000;
                let week_start = now - ((now % 86400) + 86400 * (now / 86400) % 7);
                let last_week_start = week_start - 7 * 86400;
                let last_week_end = week_start - 1;
                filters.start_time = Some(last_week_start * 1000);
                filters.end_time = Some(last_week_end * 1000);
                i += 1; // Skip the next word
            } else if word == "this" && i + 1 < words.len() && words[i + 1] == "month" {
                let now = ic_cdk::api::time() / 1_000_000;
                let day = 86400;
                let month_start = now - ((now % day) + (now / day % 30) * day);
                filters.start_time = Some(month_start * 1000);
                filters.end_time = Some(now * 1000);
                i += 1; // Skip the next word
            }
            // Content type filters
            else if word == "with" && i + 1 < words.len() && words[i + 1] == "attachments" {
                filters.has_attachments = true;
                i += 1; // Skip the next word
            } else if word == "with" && i + 1 < words.len() && words[i + 1] == "images" {
                filters.has_attachments = true;
                filters.attachment_type = Some("image".to_string());
                i += 1; // Skip the next word
            } else if word == "with" && i + 1 < words.len() && words[i + 1] == "files" {
                filters.has_attachments = true;
                filters.attachment_type = Some("file".to_string());
                i += 1; // Skip the next word
            }
            // Message type filters
            else if word == "replies" || word == "in:replies" {
                filters.is_reply = true;
            } else if word == "threads" || word == "in:threads" {
                filters.in_thread = true;
            } else if word == "edited" {
                filters.is_edited = true;
            }
            // From specific senders
            else if word.starts_with("from:") {
                let sender = word.strip_prefix("from:").unwrap_or("");
                if !sender.is_empty() {
                    filters.sender_name = Some(sender.to_string());
                }
            }
            // In specific conversations
            else if word.starts_with("in:") {
                let conversation = word.strip_prefix("in:").unwrap_or("");
                if !conversation.is_empty() {
                    filters.conversation_id = Some(conversation.to_string());
                }
            }
            // Sort options
            else if word == "sort:time" {
                filters.sort_by = SortField::Timestamp;
            } else if word == "sort:platform" {
                filters.sort_by = SortField::Platform;
            } else if word == "sort:relevance" {
                filters.sort_by = SortField::Relevance;
            } else if word == "asc" || word == "sort:asc" {
                filters.sort_direction = SortDirection::Ascending;
            } else if word == "desc" || word == "sort:desc" {
                filters.sort_direction = SortDirection::Descending;
            }
            // If not a special token, add to clean query
            else {
                if !cleaned_query.is_empty() {
                    cleaned_query.push(' ');
                }
                cleaned_query.push_str(words[i]);
            }
            
            i += 1;
        }
        
        (cleaned_query, filters)
    }
    
    // Determine if a sender name matches the filter
    pub fn matches_sender_name(&self, name: &str) -> bool {
        if let Some(ref filter_name) = self.sender_name {
            name.to_lowercase().contains(&filter_name.to_lowercase())
        } else {
            true
        }
    }
    
    // Create a Tantivy query string from the filters
    pub fn to_query_string(&self) -> String {
        let mut query_parts = Vec::new();
        
        // Platform filter
        if let Some(platform) = &self.platform {
            let platform_str = match platform {
                Platform::Telegram => "telegram",
                Platform::Slack => "slack",
                Platform::Discord => "discord",
                Platform::Twitter => "twitter",
                Platform::Facebook => "facebook",
                Platform::WhatsApp => "whatsapp",
            };
            query_parts.push(format!("platform:{}", platform_str));
        }
        
        // Time range filters
        if let Some(start) = self.start_time {
            query_parts.push(format!("timestamp:>={}", start));
        }
        
        if let Some(end) = self.end_time {
            query_parts.push(format!("timestamp:<={}", end));
        }
        
        // Conversation filter
        if let Some(ref conv_id) = self.conversation_id {
            query_parts.push(format!("conversation_id:{}", conv_id));
        }
        
        // Sender ID filter
        if let Some(ref sender_id) = self.sender_id {
            query_parts.push(format!("sender_id:{}", sender_id));
        }
        
        // Message type filters
        if self.is_reply {
            query_parts.push("reply_to:*".to_string());
        }
        
        if self.in_thread {
            query_parts.push("thread_id:*".to_string());
        }
        
        if self.is_edited {
            query_parts.push("is_edited:true".to_string());
        }
        
        // Attachments filter
        if self.has_attachments {
            query_parts.push("has_attachments:true".to_string());
            
            if let Some(ref att_type) = self.attachment_type {
                query_parts.push(format!("attachment_type:{}", att_type));
            }
        }
        
        // Join all parts with AND operator
        if query_parts.is_empty() {
            "*".to_string() // Match all if no filters
        } else {
            query_parts.join(" AND ")
        }
    }
}