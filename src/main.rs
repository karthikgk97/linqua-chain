use env_logger::Builder;
use linqua_chain::vectordb_mod::base_vectordb::BaseVectordb;
use linqua_chain::vectordb_mod::qdrant_db::{QdrantDB};


#[tokio::main]
async fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    log::info!("Information message");

    let qdb_client = QdrantDB::new(None);
    let _ = qdb_client.list_available_collections().await;

    let _ = qdb_client.delete_collection("sports_collect").await;

}
