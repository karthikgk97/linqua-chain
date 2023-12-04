use crate::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use crate::vectordb_mod::qdrant_db::{QdrantDBStruct};
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CDOMStruct{
    qdrantdb_instance: QdrantDBStruct
}

#[async_trait]
impl BaseVectorDBTrait for CDOMStruct{
    type VecDBDataType = Vec<String>;

    fn new(vectordb_url: Option<&str>, embeddings_model_name: Option<&str>) -> Self{
        return CDOMStruct{
            qdrantdb_instance: QdrantDBStruct::new(None, None)
        };
    }

    // function for listing available collections
    async fn list_available_collections(&self){
        let _ = self.qdrantdb_instance.list_available_collections().await;
    }

    // function for deleting the given collection
    async fn delete_collection(&self, collection_to_delete: &str){
        let _ = self.qdrantdb_instance.delete_collection(collection_to_delete).await;
    }

    // function for creating a collection
    async fn create_collection(&self, collection_name: &str){
        let _ = self.qdrantdb_instance.create_collection(collection_name).await;
    }

    // function for adding data to a collection
    async fn add_stuff_to_collection(&self, collection_name: &str, stuff_to_add: Self::VecDBDataType, id_for_stuff: Vec<Uuid>, filter_for_stuff: Vec<HashMap<String, String>>){
        self.qdrantdb_instance.add_stuff_to_collection(collection_name, stuff_to_add, id_for_stuff, filter_for_stuff).await;
    }

    // function for querying a collection
    async fn search_collection(&self, collection_name: &str, search_query: &str, search_filter: Option<HashMap<&str, &str>>, search_limit: u64) -> Vec<HashMap<String, f64>>{
        return self.qdrantdb_instance.search_collection(collection_name, search_query, search_filter, search_limit).await;
    }
}

