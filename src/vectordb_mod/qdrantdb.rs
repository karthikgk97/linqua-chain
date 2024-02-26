// file containing code for QdrantDB

use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, PointStruct, SearchPoints, VectorParams, VectorsConfig,
};
use std::collections::{self, HashMap};
use uuid::Uuid;

// importing crate files
use crate::core::embedding_config::EmbeddingConfig;
use crate::core::vectordb_config::{EmbeddingDistanceType, VectorDBClientType, VectorDBConfig};
use crate::embeddings_mod::fast_embed::FastEmbedStruct;

pub struct QdrantDBStruct;

pub enum QdrantDBErrors {
    CollectionAlreadyExistsError,
    CreateCollectionError,
}

impl QdrantDBStruct {
    pub fn get_vectordb_config(
        vectordb_url: String,
        embedding_config: EmbeddingConfig,
        distance_type: EmbeddingDistanceType,
    ) -> VectorDBConfig {
        let vectordb_client = QdrantClient::from_url(&vectordb_url).build().unwrap();

        VectorDBConfig {
            vectordb_client: VectorDBClientType::Qdrant(vectordb_client),
            distance_type,
            embedding_config,
        }
    }

    pub async fn list_available_collections(vectordb_config: &VectorDBConfig) -> Vec<String> {
        log::info!("Listing available collections");
        match &vectordb_config.vectordb_client {
            VectorDBClientType::Qdrant(qdrant_client) => {
                let available_collections = qdrant_client.list_collections().await.unwrap();
                let extracted_names_response: Vec<String> = available_collections
                    .collections
                    .iter()
                    .map(|x| x.name.clone())
                    .collect();
                return extracted_names_response;
            }
        }
    }

    pub async fn create_collection(
        vectordb_config: VectorDBConfig,
        collection_name: &str,
    ) -> Result<String, QdrantDBErrors> {
        let all_collections: Vec<String> = Self::list_available_collections(&vectordb_config).await;
        if all_collections.contains(&collection_name.to_string()) {
            log::error!(
                "Cannot create collection {} as it already exists",
                collection_name
            );
            return Err(QdrantDBErrors::CollectionAlreadyExistsError);
        } else {
            log::info!("Creating collection {}", collection_name);

            match &vectordb_config.vectordb_client {
                VectorDBClientType::Qdrant(qdrant_client) => {
                    let create_collection_response = qdrant_client
                        .create_collection(&CreateCollection {
                            collection_name: collection_name.to_string(),
                            vectors_config: Some(VectorsConfig {
                                config: Some(Config::Params(VectorParams {
                                    size: vectordb_config.embedding_config.embedding_model_dimension
                                        as u64,
                                    distance: Distance::Cosine.into(),
                                    ..Default::default()
                                })),
                            }),
                            ..Default::default()
                        })
                        .await
                        .unwrap();

                    match create_collection_response.result {
                        true => {
                            log::info!("Collection {} creation Successful", collection_name);
                            return Ok(String::from("Collection created successfully"));
                        }
                        false => {
                            log::error!("Collection {} creation Unsuccessful", collection_name);
                            return Err(QdrantDBErrors::CreateCollectionError);
                        }
                    }
                }
            }
        }
    }
}
