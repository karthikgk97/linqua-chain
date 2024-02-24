use linqua_chain::embeddings_mod::fast_embed::FastEmbedStruct;
use linqua_chain::core::embedding_config::EmbeddingModelObject;

fn main(){
    
    println!("Main Function");

    let emb_struct = FastEmbedStruct::new(String::from("BGEBaseENV15"));

    let documents = vec![
        "passage: Hello, World!",
         "query: Hello, World!",
         "passage: This is an example passage."
    ];
    
    println!("Model Size {:?}", emb_struct.embedding_model_config.embedding_model_dimension);
    match &emb_struct.embedding_model_config.embedding_model_object {
        EmbeddingModelObject::FastEmbed(embedding_model) => {
            if let Ok(embeddings) = embedding_model.embed(documents, None){
                println!("Embeddings length: {}", embeddings.len());
                println!("Dimension is {}", embeddings[0].len());
            } else {
                println!("Failed to Embed");
            }
        },
    }
}

