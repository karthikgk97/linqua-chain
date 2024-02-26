// File for embedding config
use fastembed::TextEmbedding;


pub enum EmbeddingModelObject{
    FastEmbed(TextEmbedding)
}

pub struct EmbeddingConfig {
    pub embedding_model_object: EmbeddingModelObject,
    pub embedding_model_type: String,
    pub embedding_model_name: String,
    pub embedding_model_dimension: usize,
}
