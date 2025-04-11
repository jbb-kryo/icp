use crate::{Error, Result};
use std::collections::HashMap;
use std::cell::RefCell;

// Vector dimensions for simple embeddings
const EMBEDDING_DIMENSIONS: usize = 128;

// Thread-local storage for cached embeddings
thread_local! {
    static EMBEDDING_CACHE: RefCell<HashMap<String, Vec<f32>>> = RefCell::new(HashMap::new());
}

// Simple embedding model
// In a real implementation, this would use a proper embedding model like OpenAI's text-embedding-ada-002
// For this demo, we use a simple hashing approach
pub fn embed_text(text: &str) -> Vec<f32> {
    // Check cache first
    let cached = EMBEDDING_CACHE.with(|cache| {
        cache.borrow().get(text).cloned()
    });
    
    if let Some(cached_embedding) = cached {
        return cached_embedding;
    }
    
    // Create a zero vector with the correct dimensions
    let mut embedding = vec![0.0; EMBEDDING_DIMENSIONS];
    
    // Tokenize the text into words
    let tokens: Vec<&str> = text
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    
    // For each token, update the embedding
    for token in tokens {
        let token_hash = simple_hash(token);
        let position = token_hash % EMBEDDING_DIMENSIONS;
        embedding[position] += 1.0;
    }
    
    // Normalize the vector
    let magnitude = (embedding.iter().map(|&x| x * x).sum::<f32>()).sqrt();
    if magnitude > 0.0 {
        for value in &mut embedding {
            *value /= magnitude;
        }
    }
    
    // Cache the result
    EMBEDDING_CACHE.with(|cache| {
        cache.borrow_mut().insert(text.to_string(), embedding.clone());
    });
    
    embedding
}

// Calculate cosine similarity between two vectors
pub fn calculate_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return 0.0;
    }
    
    let dot_product: f32 = vec1.iter().zip(vec2.iter())
        .map(|(&a, &b)| a * b)
        .sum();
    
    dot_product
}

// Simple string hashing function
fn simple_hash(s: &str) -> usize {
    let mut hash: usize = 0;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
    }
    hash
}

// Clear the embedding cache
pub fn clear_cache() {
    EMBEDDING_CACHE.with(|cache| {
        cache.borrow_mut().clear();
    });
}

// Get the current cache size
pub fn get_cache_size() -> usize {
    EMBEDDING_CACHE.with(|cache| {
        let cache = cache.borrow();
        cache.len()
    })
}