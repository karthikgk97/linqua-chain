use qdrant_client::client::QdrantClient;
use crate::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use crate::embeddings_mod::base_embeddings::BaseEmbeddingsTrait;
use async_trait::async_trait;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    PointId, PointStruct, Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig
};

use crate::embeddings_mod::fast_embed::{FastEmbedStruct};

use std::collections::HashMap;

pub struct QdrantDBStruct{
    client: QdrantClient,
    pub embeddings_model: FastEmbedStruct
}

#[async_trait]
impl BaseVectorDBTrait for QdrantDBStruct{
    type VecDBDataType = Vec<PointStruct>;
    type FilterDataType = Option<Filter>;

    fn new(vectordb_url: Option<&str>, embeddings_model_name: Option<&str>) -> Self{
        log::info!("Initializing new instance for QdrantDB");
        let default_qdrant_url = "http://localhost:6334".to_string();

        return QdrantDBStruct{
            client: QdrantClient::from_url(vectordb_url.unwrap_or(&default_qdrant_url)).build().unwrap(),
            embeddings_model: FastEmbedStruct::new(embeddings_model_name)
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

        match delete_collection_response.result{
            true => {
                log::info!("Collection {} deletion Successful", collection_to_delete);
            }
            false => {
                log::warn!("Collection {} deletion Unsuccessful", collection_to_delete);
            }
        }
        log::debug!("Qdrant's Time taken:: for deleting collection {} is {}", collection_to_delete, delete_collection_response.time);
    }

    async fn create_collection(&self, collection_name: &str){
        if self.client.has_collection(collection_name).await.unwrap(){
            log::warn!("Collection name {} already exists!", collection_name);
        }
        else{
            log::info!("Creating collection {}", collection_name);
            
            let create_collection_response = self.client.create_collection(&CreateCollection {
                collection_name: collection_name.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: self.embeddings_model.get_current_model_size(),
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await.unwrap();
            
            match create_collection_response.result{
                true => {
                    log::info!("Collection {} creation Successful", collection_name);
                }
                false => {
                    log::warn!("Collection {} creation Unsuccessful", collection_name);
                }
            }

            log::debug!("Qdrant's Time taken:: for creating collection {} is {}", collection_name, create_collection_response.time);
        }    
    }

    async fn add_stuff_to_collection(&self, collection_name: &str, stuff_to_add : Self::VecDBDataType){
        log::info!("Adding data to collection {}", collection_name);
        let add_to_collection_response = self.client.upsert_points_blocking(collection_name, stuff_to_add, None).await.unwrap();

        log::info!("Add stuff to collection {} response: {:?}", collection_name, add_to_collection_response.result);
           
        log::debug!("Qdrant's Time taken:: for adding stuff to collection {} is {}", collection_name, add_to_collection_response.time);
    }

    async fn search_collection(&self, collection_name: &str, vec_to_search: Vec<f32>, search_filter: Self::FilterDataType, search_limit: u64) -> Vec<HashMap<String, f64>>{
        let search_result_response = self.client.search_points(&SearchPoints {
            collection_name: collection_name.to_string(),
            vector: vec_to_search,
            filter: search_filter,
            limit: search_limit,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await.unwrap();

        let mut search_output: Vec<HashMap<String, f64>> = Vec::new();
        for search_result_idx in  0..search_result_response.result.len(){
            let mut result_hashmap: HashMap<String, f64> = HashMap::new();
            result_hashmap.insert(search_result_response.result[search_result_idx].payload["document_for_embeddings"].to_string(), search_result_response.result[search_result_idx].score.into());
            search_output.push(result_hashmap);
        }
        log::info!("Search output is {:?}", search_output);
        return search_output;
    }
    
}

impl QdrantDBStruct{
    pub fn create_empty_payload() -> Payload{
        return Payload::new();
    }

    fn create_payload_data(mut payload_to_add: Payload, payload_key: &str, payload_value: &str) -> Payload{
        payload_to_add.insert(payload_key.to_string(), payload_value.to_string());
    
        return payload_to_add;
    }

    fn create_point_struct(id_num: PointId, embeddings_data: Vec<f32>, payload_data:Payload) -> PointStruct{
        return PointStruct::new(id_num, embeddings_data, payload_data);
    }

    fn create_query_filter(filter_condition: &str, filter_key: &str, filter_value: &str) -> Option<Filter>{
        log::info!("Adding Filter for condition {}", filter_condition);
        match filter_condition{
            "all" => {
                return Some(Filter::all(
                    [
                        Condition::matches(
                            filter_key,
                            filter_value.to_string()
                        )
                    ]
                ))
            },
            "any" => {
                return Some(Filter::any(
                    [
                        Condition::matches(
                            filter_key,
                            filter_value.to_string()
                        )
                    ]
                ))
            },
            _ => {
                log::error!("Filter condition not found. Choose either any or all");
                return None;
            }
        }
        
    }

    // fn 
}