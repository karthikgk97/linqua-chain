// file containing fast embed code
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use crate::core::embedding_config::{EmbeddingDistanceType, EmbeddingConfig, EmbeddingModelObject};

pub struct FastEmbedStruct {
    pub embedding_model_config: EmbeddingConfig
}


impl FastEmbedStruct {
    fn  get_embedding_model_from_string(embedding_model_name: &str) -> EmbeddingModel {
        let model = match embedding_model_name {
            "AllMiniLML6V2" => EmbeddingModel::AllMiniLML6V2,
            "BGEBaseENV15" => EmbeddingModel::BGEBaseENV15,
            "BGESmallENV15" => EmbeddingModel::BGESmallENV15,
            "BGELargeENV15" => EmbeddingModel::BGELargeENV15,
            _ => EmbeddingModel::BGEBaseENV15
        };

        model
    }

    pub fn new(embedding_model_name: String)  -> Self{
        // setting the embedding model object
        let emb_model: TextEmbedding = TextEmbedding::try_new(InitOptions {
            model_name: Self::get_embedding_model_from_string(&embedding_model_name),
            show_download_progress: true,
            ..Default::default()
        }).expect("Unable to set the FastEmbed TextEmbedding Object");

        let available_embedding_models = TextEmbedding::list_supported_models();
        let mut model_dimension: usize = 0;
        available_embedding_models.iter().for_each(|x| {
            if x.model == Self::get_embedding_model_from_string(&embedding_model_name) {
                model_dimension = x.dim;
            }
        }
        );

        let emb_model_config: EmbeddingConfig = EmbeddingConfig{
            embedding_model_object: EmbeddingModelObject::FastEmbed(emb_model),
            embedding_model_type: String::from("FastEmbed"),
            embedding_model_name: embedding_model_name,
            embedding_model_dimension: model_dimension,
            embedding_distance_type: EmbeddingDistanceType::Cosine
        };

        // returning the fast embed struct
        FastEmbedStruct {
            embedding_model_config: emb_model_config
        }
    }
}
