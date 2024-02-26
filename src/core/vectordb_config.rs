// file containing core enums and structs
use crate::core::embedding_config::EmbeddingConfig;
use qdrant_client::client::QdrantClient;

#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddingDistanceType {
    Euclidean,
    Dot,
    Cosine
}

pub enum VectorDBClientType {
    Qdrant(QdrantClient)
}

pub struct VectorDBConfig{
    pub vectordb_client: VectorDBClientType,
    pub distance_type: EmbeddingDistanceType,
    pub embedding_config: EmbeddingConfig
}
