use env_logger::Builder;
use std::collections::HashMap;
use maplit::hashmap;
use linqua_chain::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use linqua_chain::vectordb_mod::qdrant_db::{QdrantDBStruct};

#[tokio::main]
async fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let qdb_client = QdrantDBStruct::new(None, None);
    let _ = qdb_client.list_available_collections().await;

    let _ = qdb_client.delete_collection("test_collection").await;

    let _ = qdb_client.create_collection("test_collection").await;
    let documents = vec![
        "this is test document 1".to_string(),
        "this is test document 2".to_string(),
        "this is test document 3".to_string(),
        "this is test document 4".to_string(),
        "this is test document 5".to_string(),
        "this is test document 6".to_string()
        ];

    let metadata_for_stuff: Vec<HashMap<String, String>>= vec![
        hashmap!{"document_name".to_string() => "test".to_string()},
        hashmap!{"document_name".to_string() => "test".to_string()},
        hashmap!{"document_name".to_string() => "test".to_string()},
        hashmap!{"document_name".to_string() => "dummy".to_string()},
        hashmap!{"document_name".to_string() => "dummy".to_string()},
        hashmap!{"document_name".to_string() => "dummy".to_string()}
        ];

    let id_for_stuff: Vec<u64> = vec![1, 2, 3, 4, 5, 6];

    let _ = qdb_client.add_stuff_to_collection("test_collection", documents, id_for_stuff, metadata_for_stuff).await;

    let _ = qdb_client.search_collection("test_collection", "document 1 ", Some(hashmap!{"document_name" => "test"}), 5).await;

}