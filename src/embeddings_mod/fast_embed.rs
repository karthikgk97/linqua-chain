// file containing fast embed code
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use crate::core::embedding_config::{EmbeddingConfig, EmbeddingModelObject};

/// A struct that contains embedding config for FastEmbed library
pub struct FastEmbedStruct {
    pub embedding_config: EmbeddingConfig
}

#[derive(Debug)]
pub enum FastEmbeddingError{
    ModelNotFound
}

impl FastEmbedStruct {
    /// private function for matching the embedding model based on string
    fn  get_embedding_model_from_string(embedding_model_name: &str) -> Result<EmbeddingModel, FastEmbeddingError> {
        let model = match embedding_model_name {
            "AllMiniLML6V2" => EmbeddingModel::AllMiniLML6V2,
            "BGEBaseENV15" => EmbeddingModel::BGEBaseENV15,
            "BGESmallENV15" => EmbeddingModel::BGESmallENV15,
            "BGELargeENV15" => EmbeddingModel::BGELargeENV15,
            _ => return Err(FastEmbeddingError::ModelNotFound)
        };

        Ok(model)
    }

    /// Constructs a new fast embed struct 
    pub fn new(embedding_model_name: String)  -> Result<Self, FastEmbeddingError>{
        log::info!("Initiating FastEmbed Struct with embedding model name {}", &embedding_model_name);

        
        // making sure the embedding model name exist
        let embedding_model = match Self::get_embedding_model_from_string(&embedding_model_name) {
            Ok(model) => model,
            Err(err) => return Err(err)
        };

        // setting the embedding model object
        let emb_model: TextEmbedding = TextEmbedding::try_new(InitOptions {
            model_name: embedding_model.clone(), 
            show_download_progress: true,
            ..Default::default()
        }).expect("Unable to set the FastEmbed TextEmbedding Object");

        let available_embedding_models = TextEmbedding::list_supported_models();
        let mut model_dimension: usize = 0;
        available_embedding_models.iter().for_each(|x| {
            if x.model == embedding_model {
                log::debug!("Found the embedding model from available models. Retrieving its dimension");
                model_dimension = x.dim;
            }
        }
        );
        

        let emb_config: EmbeddingConfig = EmbeddingConfig{
            embedding_model_object: EmbeddingModelObject::FastEmbed(emb_model),
            embedding_model_type: String::from("FastEmbed"),
            embedding_model_name,
            embedding_model_dimension: model_dimension,
        };

        // returning the fast embed struct
        Ok(FastEmbedStruct {
            embedding_config: emb_config
        })
    }
}
