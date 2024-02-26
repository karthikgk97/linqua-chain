// file for containing modules list

// core elements
pub mod core {
    pub mod llm_config;
    pub mod embedding_config;
    pub mod vectordb_config;
}

// including embedding mod
pub mod embeddings_mod {
    pub mod fast_embed;
}

// including llm mod 
pub mod llm_mod {
    pub mod openai;
}

// including vectorDB mod
pub mod vectordb_mod {
    pub mod qdrantdb;
}
