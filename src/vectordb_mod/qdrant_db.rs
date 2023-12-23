use qdrant_client::client::QdrantClient;
use crate::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use crate::embeddings_mod::base_embeddings::BaseEmbeddingsTrait;
use async_trait::async_trait;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    PointStruct, Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig
};
use uuid::Uuid;
// use sha256::{digest}

use crate::embeddings_mod::fast_embed::{FastEmbedStruct};

use std::collections::HashMap;

pub struct QdrantDBStruct{
    client: QdrantClient,
    pub embeddings_model: FastEmbedStruct
}

#[async_trait]
impl BaseVectorDBTrait for QdrantDBStruct{
    // type VecDBDataType = Vec<PointStruct>;
    type VecDBDataType = Vec<String>;

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

    async fn add_stuff_to_collection(&self, collection_name: &str, stuff_to_add: Self::VecDBDataType, id_for_stuff: Vec<Uuid>, metadata_for_stuff: Vec<HashMap<String, String>>){
        
        if !(stuff_to_add.len() == id_for_stuff.len() && id_for_stuff.len() == metadata_for_stuff.len()) {
            log::error!("Vectors should be of same length!");
            return;
        }

        log::info!("Adding data to collection {}", collection_name);

        let converted_vectors = QdrantDBStruct::convert_to_pointstruct(&self.embeddings_model, stuff_to_add, id_for_stuff, metadata_for_stuff);
        let add_to_collection_response = self.client.upsert_points_batch_blocking(collection_name, converted_vectors, None, 100).await.unwrap();
        log::info!("Add stuff to collection {} response: {:?}", collection_name, add_to_collection_response.result);
           
        log::debug!("Qdrant's Time taken:: for adding stuff to collection {} is {}", collection_name, add_to_collection_response.time);
    }

    async fn search_collection(&self, collection_name: &str, search_query: &str, search_filter: Option<HashMap<&str, &str>>, search_limit: u64) -> HashMap<String, f64>{
        
        let vec_to_search = &self.embeddings_model.embed_stuff(vec![search_query.to_string()])[0];

        let mut qdrant_filter = None;
        if search_filter.is_some() {
            qdrant_filter = QdrantDBStruct::create_query_filter("all", search_filter.unwrap());
        }

        let search_result_response = self.client.search_points(&SearchPoints {
            collection_name: collection_name.to_string(),
            vector: vec_to_search.clone(),
            filter: qdrant_filter,
            limit: search_limit,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await.unwrap();
        
        let output_hashmap : HashMap<String, f64> = search_result_response.result.iter().map(|x| {
            let without_quotes = x.payload["document_for_embeddings"].as_str().unwrap().trim_matches('"').to_string();
            return (without_quotes, x.score.into());
        }).collect();

        return output_hashmap;
    }
}

impl QdrantDBStruct{
    fn create_empty_payload() -> Payload{
        return Payload::new();
    }

    fn add_to_existing_payload(mut payload_to_add: Payload, payload_key: &str, payload_value: &str) -> Payload{
        payload_to_add.insert(payload_key.to_string(), payload_value.to_string());
    
        return payload_to_add;
    }

    fn create_payload_data(payload_map: HashMap<String, String>) -> Payload{
        let mut tmp_payload = Self::create_empty_payload();
        for (payload_key, payload_value) in payload_map.iter(){
            tmp_payload.insert(payload_key.to_string(), payload_value.to_string());
        }

        return tmp_payload;
    }

    fn create_point_struct(id_num: Uuid, embeddings_data: Vec<f32>, payload_data:Payload) -> PointStruct{
        return PointStruct::new(id_num.to_string(), embeddings_data, payload_data);
    }

    fn convert_to_pointstruct(embeddings_model: &FastEmbedStruct, doc_to_pointstruct: Vec<String>, id_for_pointstruct: Vec<Uuid>, metadata_for_pointstruct: Vec<HashMap<String, String>> ) -> Vec<PointStruct>{
        let mut tmp_vector_store: Vec<PointStruct> = Vec::new();
        for doc_idx in 0..doc_to_pointstruct.len(){
            let embeddings_data = &embeddings_model.embed_stuff(vec![doc_to_pointstruct[doc_idx].clone()])[0];
            let mut payload_data = Self::create_payload_data(metadata_for_pointstruct[doc_idx].clone());
            // add document as payload
            payload_data = Self::add_to_existing_payload(payload_data, "document_for_embeddings", &doc_to_pointstruct[doc_idx]);
            let id_data = id_for_pointstruct[doc_idx];
            tmp_vector_store.push(Self::create_point_struct(id_data.into(), embeddings_data.clone(), payload_data));
        }
        return tmp_vector_store;
    }

    fn create_qdrant_condition(condition_key: &str, condition_value: &str) -> Condition{
        return Condition::matches(condition_key, condition_value.to_string());
    }

    fn create_query_filter(filter_type: &str, filter_conditions: HashMap<&str, &str>) -> Option<Filter>{
        log::info!("Adding Filter for condition {:?}", filter_conditions);

        let mut conditions: Vec<Condition> =  Vec::new();

        for (condition_key, condition_value) in filter_conditions.iter(){
            conditions.push(Self::create_qdrant_condition(condition_key, condition_value));
        }

        match filter_type{
            "all" => {
                return Some(Filter::all(conditions))
            },
            "any" => {
                return Some(Filter::any(conditions))
            },
            _ => {
                log::error!("Filter condition not found. Choose either any or all");
                return None;
            }
        }
        
    }

    pub fn create_ids(text_vec_for_uuid: Vec<String>) -> Vec<Uuid>{

        let uuid_list: Vec<Uuid> = text_vec_for_uuid.iter().map(|x| {
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, x.as_bytes());
            return uuid;
        }).collect();

        return uuid_list;
    }

}