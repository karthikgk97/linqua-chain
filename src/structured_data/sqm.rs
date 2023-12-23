use crate::llm_mod::base_llm::BaseLLMTrait;
use crate::llm_mod::openai_llm::OpenAILLMStruct;
use crate::llm_mod::ollama_llm::OllamaLLMStruct;
use serde_yaml::Value;
use crate::llm_mod::base_llm::ModelType;

enum LanguageModelVariant{
    OpenAI(OpenAILLMStruct),
    Ollama(OllamaLLMStruct)
}

pub struct SQMStruct{
    llm_instance: LanguageModelVariant,
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


impl SQMStruct{

    pub fn new(model_name: ModelType, track_history: bool) -> Self {

            let llm_instance = match model_name {
                ModelType::OpenAI => LanguageModelVariant::OpenAI(OpenAILLMStruct::new(Some("gpt-3.5-turbo"))),
                ModelType::Ollama => LanguageModelVariant::Ollama(OllamaLLMStruct::new(Some("llama"), track_history)),
            };
    
            return SQMStruct{
                llm_instance
            };
    }


    pub fn set_system_prompt(&mut self, system_prompt_message: &str) {
        // self.llm_instance.set_system_prompt(system_prompt_message);
        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => openai_llm.set_custom_system_prompt(system_prompt_message),
            LanguageModelVariant::Ollama(_) => {
                log::info!("test");
            }
    }
    }

    pub fn set_temperature(&mut self, new_temperature: f32) {
        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => openai_llm.set_temperature(new_temperature),
            LanguageModelVariant::Ollama(_) => {
                log::info!("test");
                // Handle Ollama case if needed
            }
    }}

    pub fn set_top_p(&mut self, new_top_p: f32) {
        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => openai_llm.set_top_p(new_top_p),
            LanguageModelVariant::Ollama(_) => {
                log::info!("test");
                // Handle Ollama case if needed
            }
    }}

    pub fn set_top_k(&mut self, new_top_k: u32) {
        log::info!("Need to implement top_k");

    }

    pub fn set_max_output_length(&mut self, new_max_output_length: i32) {
        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => openai_llm.set_max_output_length(new_max_output_length),
            LanguageModelVariant::Ollama(_) => {
                log::info!("test");
                // Handle Ollama case if needed
            }
        }
    }


    pub async fn chat(&mut self, user_prompt: &str) -> String {

        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => {
                let mut chat_response = openai_llm.chat(user_prompt).await;
                return chat_response;
            },
            LanguageModelVariant::Ollama(_) => {
                let mut chat_response = "test".to_string();
                return chat_response;
                // Handle Ollama case if needed
            }
        }        
    }

    pub fn clear_chat(&mut self){
        // self.llm_instance.clear_chat();
        match &mut self.llm_instance {
            LanguageModelVariant::OpenAI(openai_llm) => {
                let mut chat_response = openai_llm.clear_chat();
            },
            LanguageModelVariant::Ollama(_) => {
                // let mut chat_response = "test";
                // Handle Ollama case if needed
            }
        }
    }
// }

// impl SQMStruct{

    pub fn get_default_system_message(&self, table_name: &str, table_description: &str, available_columns_list: Vec<String>) -> String {

        // Read the YAML file content
        let yaml_content = std::fs::read_to_string("src/default_prompts_store/sqm.yml").expect("Error reading the file");

        // Parse the YAML content
        let yaml_value: Value = serde_yaml::from_str(&yaml_content).expect("Error parsing YAML");

        // Extract the 'prompt' value
        let mut prompt = yaml_value["prompt"].as_str().expect("Error extracting prompt value").to_string();

        // Convert Vec<&str> to &str using join
        let columns_list_str = format!("[\"{}\"]", available_columns_list.join("\",\""));

        prompt = prompt.replace("{table_name_var}", table_name);
        prompt = prompt.replace("{table_description_var}", table_description);
        prompt = prompt.replace("{available_columns_for_table_var}", &columns_list_str);

        return prompt;
    }

    pub fn string_to_list(&self, string_to_convert: String) -> Vec<String>{
        let mut string_to_convert = string_to_convert;
        string_to_convert = string_to_convert.replace("[", "");
        string_to_convert = string_to_convert.replace("]", "");
        let converted_list: Vec<String> = string_to_convert.split(",").map(|x| x.trim().trim_matches('"').to_string()).collect();
        return converted_list;
    }
}
