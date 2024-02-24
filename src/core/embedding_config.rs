// File for embedding config
use fastembed::TextEmbedding;


pub enum EmbeddingModelObject{
    FastEmbed(TextEmbedding)
}


pub enum EmbeddingDistanceType {
    Euclidean,
    Dot,
    Cosine
}


pub struct EmbeddingConfig {
    pub embedding_model_object: EmbeddingModelObject,
    pub embedding_model_type: String,
    pub embedding_model_name: String,
    pub embedding_model_dimension: usize,
    pub embedding_distance_type: EmbeddingDistanceType 
}
