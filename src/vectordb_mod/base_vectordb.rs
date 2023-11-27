
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait BaseVectorDBTrait{
    type VecDBDataType;
    type FilterDataType;

    // function for setting up the client
    // fn setup_vectordb_client();
    fn new(vectordb_url: Option<&str>, embeddings_model_name: Option<&str>) -> Self;

    // function for listing available collections
    async fn list_available_collections(&self);

    // function for deleting the given collection
    async fn delete_collection(&self, collection_to_delete: &str);

    // function for creating a collection
    async fn create_collection(&self, collection_name: &str);

    // function for adding data to a collection
    // async fn add_stuff_to_collection(&self, collection_name: &str, stuff_to_add: Self::VecDBDataType);
    async fn add_stuff_to_collection(&self, collection_name: &str, stuff_to_add: Self::VecDBDataType, id_for_stuff: Vec<u64>, filter_for_stuff: Vec<HashMap<String, String>>);

    // function for querying a collection
    async fn search_collection(&self, collection_name: &str, search_query: &str, search_filter: Option<HashMap<&str, &str>>, search_limit: u64) -> Vec<HashMap<String, f64>>;
}