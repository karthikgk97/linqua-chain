use env_logger::Builder;
use linqua_chain::embeddings_mod::fast_embed::FastEmbedStruct;
use linqua_chain::core::embedding_config::EmbeddingModelObject;

fn main(){
    Builder::new().filter_level(log::LevelFilter::Info).init(); 
    log::info!("Main Function");

    let emb_struct = FastEmbedStruct::new(String::from("BGESmallENV15")).unwrap();

    let documents = vec![
        "passage: Hello, World!",
         "query: Hello, World!",
         "passage: This is an example passage."
    ];
    
    log::info!("Model Size {:?}", emb_struct.embedding_config.embedding_model_dimension);
    match &emb_struct.embedding_config.embedding_model_object {
        EmbeddingModelObject::FastEmbed(embedding_model) => {
            if let Ok(embeddings) = embedding_model.embed(documents, None){
                log::info!("Embeddings length: {}", embeddings.len());
                log::info!("Dimension is {}", embeddings[0].len());
            } else {
                log::error!("Failed to Embed");
            }
        },
    }
}

