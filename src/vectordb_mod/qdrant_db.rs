use qdrant_client::client::QdrantClient;
use crate::vectordb_mod::base_vectordb::BaseVectordb;
use async_trait::async_trait;
pub struct QdrantDB{
    client: QdrantClient,
}

#[async_trait]
impl BaseVectordb for QdrantDB{
    fn new(vectordb_url: Option<&str>) -> Self{
        log::info!("Initializing new instance for QdrantDB");
        let default_qdrant_url = "http://localhost:6334".to_string();
        // let client = QdrantClient::from_url(vectordb_url.unwrap_or(&default_qdrant_url)).build().unwrap();

        // return client;
        return QdrantDB{
            client: QdrantClient::from_url(vectordb_url.unwrap_or(&default_qdrant_url)).build().unwrap()
        };
    }

    async fn list_available_collections(&self){
        log::info!("Listing available collections");
        let list_collection_response = self.client.list_collections().await.unwrap();
        let (available_collections, qdrant_time_taken): (Vec<String>, Vec<f64>) = list_collection_response.collections.into_iter().map(|x| (x.name, list_collection_response.time)).unzip();
        
        log::info!("Available collections: {:?}", available_collections);
        log::debug!("Qdrant's Time taken:: for listing collections is {:?}", qdrant_time_taken[0]);
    }

    async fn delete_collection(&self, collection_to_delete: &str){
        log::info!("Deleting collection {}", collection_to_delete);
        let delete_collection_response = self.client.delete_collection(collection_to_delete).await.unwrap();

        log::info!("Deleting collection response {:?}", delete_collection_response);
    }

}
