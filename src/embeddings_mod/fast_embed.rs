use fastembed::{FlagEmbedding, InitOptions, EmbeddingModel, EmbeddingBase};
use crate::embeddings_mod::base_embeddings::BaseEmbeddingsTrait;
use std::collections::HashMap;

pub struct FastEmbedStruct{
    embeddings_model: FlagEmbedding,
    pub current_model_name: String,
    available_model_info: HashMap<String, HashMap<String, String>>
}

fn enum_to_string(model_enum: EmbeddingModel) -> String{
    match model_enum{
        EmbeddingModel::AllMiniLML6V2 => String::from("AllMiniLML6V2"),
        EmbeddingModel::BGEBaseEN => String::from("BGEBaseEN"),
        EmbeddingModel::BGEBaseENV15 => String::from("BGEBaseENV15"),
        EmbeddingModel::BGESmallEN => String::from("BGESmallEN"),
        EmbeddingModel::BGESmallENV15 => String::from("BGESmallENV15"),
        EmbeddingModel::BGESmallZH => String::from("BGESmallZH"),
        EmbeddingModel::MLE5Large => String::from("MLE5Large")
    }
}


impl BaseEmbeddingsTrait for FastEmbedStruct{
    fn new(embeddings_model: Option<&str>) -> Self{
        let current_model_name: String;

        let model_name = match embeddings_model {
            Some("AllMiniLML6V2") => {
                current_model_name = "AllMiniLML6V2".to_string();
                EmbeddingModel::AllMiniLML6V2
            },
            Some("BGEBaseEN") => {
                current_model_name = "BGEBaseEN".to_string();
                EmbeddingModel::BGEBaseEN
            },
            Some("BGEBaseENV15") => {
                current_model_name = "BGEBaseENV15".to_string();
                EmbeddingModel::BGEBaseENV15
            },
            Some("BGESmallEN") => {
                current_model_name = "BGESmallEN".to_string();
                EmbeddingModel::BGESmallEN
            },
            Some("BGESmallENV15") => {
                current_model_name = "BGESmallENV15".to_string();
                EmbeddingModel::BGESmallENV15
            },
            Some("BGESmallZH") => {
                current_model_name = "BGESmallZH".to_string();
                EmbeddingModel::BGESmallZH
            },
            Some("MLE5Large") => {
                current_model_name = "MLE5Large".to_string();
                EmbeddingModel::MLE5Large
            },
            _ => {
                current_model_name = "BGEBaseEN".to_string();
                EmbeddingModel::BGEBaseEN
            }
        };

        let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
            model_name: model_name,
            show_download_message: true,
            ..Default::default()
        }).unwrap();

        let model_info_store_vec: HashMap<String, HashMap<String, String>> = FlagEmbedding::list_supported_models().iter().map(|x| {
            let mut inner_map = HashMap::new();
            let model_string = enum_to_string(x.model.clone());
            inner_map.insert("model_size".to_string(), x.dim.to_string());
            inner_map.insert("model_description".to_string(), x.description.clone());
            (model_string, inner_map)
        }).collect();

        return FastEmbedStruct{
            embeddings_model: model,
            current_model_name: current_model_name,
            available_model_info: model_info_store_vec
        };
    }


    fn embed_stuff(&self, stuff_to_embed: Vec<String>) -> Vec<Vec<f32>> {
        let embeddings = self.embeddings_model.embed(stuff_to_embed, None).unwrap();
        return embeddings;
    }

    fn list_available_embeddings_model() {
        log::info!("Available supported models: {:?}", FlagEmbedding::list_supported_models());
    }

    fn get_current_model_size(&self) -> u64{
        let model_size: &u64 = &self.available_model_info[&self.current_model_name]["model_size"].parse().unwrap();
        log::info!("Model size is {:?}", model_size);
        return *model_size;
    }
}
