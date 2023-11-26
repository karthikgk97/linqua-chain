use fastembed::{FlagEmbedding, InitOptions, EmbeddingModel, EmbeddingBase};
use serde_json::json;
use crate::embeddings_mod::base_embeddings::BaseEmbeddingsTrait;

pub struct FastEmbedStruct{
    embeddings_model: FlagEmbedding
}

impl BaseEmbeddingsTrait for FastEmbedStruct{

    fn new(embeddings_model: Option<&str>) -> Self{
        let model_name = match embeddings_model {
            Some("AllMiniLML6V2") => EmbeddingModel::AllMiniLML6V2,
            Some("BGEBaseEN") => EmbeddingModel::BGEBaseEN,
            Some("BGEBaseENV15") => EmbeddingModel::BGEBaseENV15,
            Some("BGESmallEN") => EmbeddingModel::BGESmallEN,
            Some("BGESmallENV15") => EmbeddingModel::BGESmallENV15,
            Some("BGESmallZH") => EmbeddingModel::BGESmallZH,
            Some("MLE5Large") => EmbeddingModel::MLE5Large,
            _ => EmbeddingModel::BGEBaseEN,
        };

        let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
            model_name: model_name,
            show_download_message: true,
            ..Default::default()
        }).unwrap();

        return FastEmbedStruct{
            embeddings_model: model
        };
    }


    fn embed_stuff(&self, stuff_to_embed: Vec<String>) -> Vec<Vec<f32>> {
        let embeddings = self.embeddings_model.embed(stuff_to_embed, None).unwrap();
        return embeddings;
    }

    fn list_available_embeddings_model() {
        println!("Available supported models: {:?}", FlagEmbedding::list_supported_models());
    }
}