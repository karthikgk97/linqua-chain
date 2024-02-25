use env_logger::Builder;
use linqua_chain::embeddings_mod::fast_embed::FastEmbedStruct;
use linqua_chain::core::embedding_config::EmbeddingModelObject;
use linqua_chain::core::llm_config::{LLMConfig, OpenAILLMModels};
use linqua_chain::llm_mod::openai::OpenAILLMConfig;

#[tokio::main]
async fn main(){
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

    let openai_config = OpenAILLMConfig::get_llm_config(OpenAILLMModels::Gpt35_4k, 0.7, 0.95, 512);

    let mut chat_history = Vec::new();
    let chat_response_result = OpenAILLMConfig::chat(openai_config, &mut chat_history, String::from("What is Valorant?")).await;

    match chat_response_result {
    Ok(chat_response) => {
        println!("Chat resp is {:?}", chat_response.output_response);
        // Access other fields if needed: chat_response.input_tokens, chat_response.output_tokens, chat_response.total_tokens
    }
    Err(error) => {
        // Handle the error case
        println!("Error: {:?}", error);
    }
}
}

