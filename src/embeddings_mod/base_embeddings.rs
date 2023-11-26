pub trait BaseEmbeddingsTrait{

    // initializer for embeddings model
    fn new(embeddings_model: Option<&str>) -> Self;

    // function for embedding stuff
    fn embed_stuff(&self, stuff_to_embed: Vec<String>) -> Vec<Vec<f32>>;

    // function for listing available embeddings models
    fn list_available_embeddings_model();

    // function for getting current embedding model's size
    fn get_current_model_size(&self) -> u64;
}