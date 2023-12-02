use async_trait::async_trait;
use crate::llm_mod::base_llm::BaseLLMTrait;
// use std::collections::HashMap;
use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext
    },
    Ollama,
};
use ollama_rs::generation::options::GenerationOptions;

pub struct OLlamaLLMStruct{
    llm_model: Ollama,
    model_name: String,
    system_prompt: String,
    previous_context: Option<GenerationContext>,
    pub track_history: bool,
    ollama_options: GenerationOptions
}

#[async_trait]
impl BaseLLMTrait for OLlamaLLMStruct{

    fn new(model_endpoint: &str, model_name: Option<&str>, track_history: bool) -> Self{

        let default_model_name = "llama2";

        let ollama_options = GenerationOptions::default();

        return OLlamaLLMStruct{
            llm_model: Ollama::new(model_endpoint.to_string(), 11434),
            model_name: model_name.unwrap_or(default_model_name).to_string(),
            system_prompt: "".to_string(),
            previous_context: None,
            track_history: track_history,
            ollama_options: ollama_options
        };
    }

    fn set_system_prompt(&mut self, system_prompt_message: &str){
        self.system_prompt = system_prompt_message.to_string();
    }

    fn set_temperature(&mut self, new_temperature: f32){
        log::info!("Setting temperature as {}", new_temperature);
        self.ollama_options = self.ollama_options.clone().temperature(new_temperature);
    }

    fn set_top_p(&mut self, new_top_p: f32){
        log::info!("Setting top_p as {}", new_top_p);
        self.ollama_options = self.ollama_options.clone().top_p(new_top_p);
    }

    fn set_top_k(&mut self, new_top_k: u32){
        log::info!("Setting top_k as {}", new_top_k);
        self.ollama_options = self.ollama_options.clone().top_k(new_top_k);
    }

    fn set_max_output_length(&mut self, new_max_output_length: i32){
        log::info!("Setting num_predict as {}", new_max_output_length);
        self.ollama_options = self.ollama_options.clone().num_predict(new_max_output_length);
    }

    async fn chat(&mut self, user_question: &str){

        let user_prompt = format!("[INST] + {} + [/INST]", user_question).to_string();
        let mut ollama_generate_request = GenerationRequest::new(self.model_name.clone(), user_prompt);

        // setting the previous history as context
        if self.previous_context.is_some(){
            ollama_generate_request = ollama_generate_request.clone().context(self.previous_context.clone().unwrap());
        }
        // setting the system message
        ollama_generate_request = ollama_generate_request.clone().system(self.system_prompt.clone());

        // setting options
        ollama_generate_request = ollama_generate_request.clone().options(self.ollama_options.clone());

        log::debug!("llama request is {:?}", ollama_generate_request);

        let res = self.llm_model.generate(ollama_generate_request.clone()).await;

        if let Ok(res) = res {
            log::info!("LLM response: {}", res.response);
            if let Some(final_data) = res.final_data{
                log::debug!("LLM eval duration for question {}: {}", user_question, final_data.eval_duration);

                if self.track_history{
                    self.previous_context = Some(final_data.context);
                }
            }
        }
        else{
            log::error!("Chat api call failed!");
        }
    }
}

