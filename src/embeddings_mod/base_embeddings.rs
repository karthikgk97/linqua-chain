pub trait BaseEmbeddingsTrait{

    fn new(embeddings_model: Option<&str>) -> Self;

    fn embed_stuff(&self, stuff_to_embed: Vec<String>) -> Vec<Vec<f32>>;

    fn list_available_embeddings_model();

    fn get_current_model_size(&self) -> u64;
}