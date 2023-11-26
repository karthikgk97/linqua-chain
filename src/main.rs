use env_logger::Builder;
use qdrant_client::qdrant::PointStruct;
use serde_json::json;
use qdrant_client::prelude::Payload;

// use linqua_chain::vectordb_mod::base_vectordb::BaseVectordb;
// use linqua_chain::vectordb_mod::qdrant_db::{QdrantDB};


// #[tokio::main]
// async fn main() {
//     Builder::new().filter_level(log::LevelFilter::Info).init();
//     // Builder::new().filter_level(log::LevelFilter::max()).init();
//     log::info!("Information message");

//     let qdb_client = QdrantDB::new(None);
//     let _ = qdb_client.list_available_collections().await;

//     let _ = qdb_client.delete_collection("sports_collect_2").await;

//     let _ = qdb_client.create_collection("sports_collect_2", 768).await;

//     let emb_model = set_embeddings_model();
//     let documents = vec![
//         "ms dhoni".to_string(),
//         "virat kohli the Goat".to_string(),
//         "jamie gunn".to_string(),
//         "tom brady the Goat".to_string(),
//         "lebron james".to_string(),
//         "steph curry the Goat".to_string()
//         ];

//     let mut embed = embed_stuff(&emb_model, documents.clone());

//     let mut cricket_payload = QdrantDB::create_empty_payload();
//     cricket_payload = QdrantDB::create_payload_data(cricket_payload, "sports", "cricket");

//     let mut nfl_payload = QdrantDB::create_empty_payload();
//     nfl_payload = QdrantDB::create_payload_data(nfl_payload, "sports", "nfl");

//     let mut basketball_payload = QdrantDB::create_empty_payload();
//     basketball_payload = QdrantDB::create_payload_data(basketball_payload, "sports", "basketball");

//     let mut stuff_data = vec![QdrantDB::create_point_struct(0.into(), embed[0].clone(), QdrantDB::create_empty_payload())];

    

//     // let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
//     // let _ = qdb_client.add_stuff_to_collection("sports_collect_2", points).await;

//     for i in 0..embed.len(){
        
//         let emb = embed.remove(0);
//         if i < 2{
//             cricket_payload = QdrantDB::create_payload_data(cricket_payload, "document_for_embeddings", &documents[i]);
//             println!("Cricket payload is {:?}", cricket_payload);
//             stuff_data = vec![QdrantDB::create_point_struct((i as u64).into(), emb, cricket_payload.clone())];
//         }
//         else if i < 4{
//             nfl_payload = QdrantDB::create_payload_data(nfl_payload, "document_for_embeddings", &documents[i]);
//             stuff_data = vec![QdrantDB::create_point_struct((i as u64).into(), emb, nfl_payload.clone())];
//         }
//         else{
//             basketball_payload = QdrantDB::create_payload_data(basketball_payload, "document_for_embeddings", &documents[i]);
//             stuff_data = vec![QdrantDB::create_point_struct((i as u64).into(), emb, basketball_payload.clone())];
//         }
//         qdb_client.add_stuff_to_collection("sports_collect_2", stuff_data).await;

//     }


//     let retreive_query = vec!["Who is the goat?".to_string()];
//     let mut retrieve_embed = embed_stuff(&emb_model, retreive_query.clone());
//     let _ = qdb_client.search_collection("sports_collect_2", retrieve_embed[0].clone(), None, 5).await;

// }

use linqua_chain::embeddings_mod::base_embeddings::BaseEmbeddingsTrait;
use linqua_chain::embeddings_mod::fast_embed::FastEmbedStruct;

fn main(){
    let fmd = FastEmbedStruct::new(None);
    println!("FMd is {:?}", fmd.current_model_name);
    let m_size = fmd.get_current_model_size();
    println!("Size is {}", m_size);
}