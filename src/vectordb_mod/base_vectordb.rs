
use async_trait::async_trait;
#[async_trait]
pub trait BaseVectordb{
    // function for setting up the client
    // fn setup_vectordb_client();
    fn new(vectordb_url: Option<&str>) -> Self;

    // function for listing available collections
    async fn list_available_collections(&self);

    // function for deleting the given collection
    async fn delete_collection(&self, collection_to_delete: &str);

    // // function for creating a collection
    // fn create_collection();

    // // function for adding data to a collection
    // fn add_to_collection();

    // // function for querying a collection
    // fn query_collection();
}