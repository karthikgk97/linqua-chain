pub mod vectordb_mod {
    pub mod base_vectordb;
    pub mod qdrant_db;
}

pub mod embeddings_mod {
    pub mod base_embeddings;
    pub mod fast_embed;
}

pub mod rdbms_mod {
    pub mod base_rdbms;
    pub mod polars_data;
}

pub mod llm_mod {
    pub mod base_llm;
    pub mod ollama_llm;
}

pub mod structured_data {
    pub mod ccm;
}

pub mod crate_utils;