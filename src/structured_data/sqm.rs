use crate::llm_mod::base_llm::BaseLLMTrait;
use crate::llm_mod::ollama_llm::OllamaLLMStruct;
use serde_yaml::Value;
use async_trait::async_trait;

pub struct SQMStruct{
    ollama_instance: OllamaLLMStruct,
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[async_trait]
impl BaseLLMTrait for SQMStruct{

    fn new(model_endpoint: &str, model_name: Option<&str>, track_history: bool) -> Self {

        return SQMStruct{
            ollama_instance: OllamaLLMStruct::new(model_endpoint, model_name, track_history)
        };
    }

    fn set_system_prompt(&mut self, system_prompt_message: &str) {
        self.ollama_instance.set_system_prompt(system_prompt_message);
    }

    fn set_temperature(&mut self, new_temperature: f32) {
        self.ollama_instance.set_temperature(new_temperature);
    }

    fn set_top_p(&mut self, new_top_p: f32) {
        self.ollama_instance.set_top_p(new_top_p);
    }

    fn set_top_k(&mut self, new_top_k: u32) {
        self.ollama_instance.set_top_k(new_top_k);
    }

    fn set_max_output_length(&mut self, new_max_output_length: i32) {
        self.ollama_instance.set_max_output_length(new_max_output_length);
    }

    async fn chat(&mut self, user_prompt: &str) -> String {
        let mut chat_response = self.ollama_instance.chat(user_prompt).await;
        chat_response = chat_response.replace("[INST/LLM]", "");
        return chat_response;
    }

    fn clear_chat(&mut self){
        self.ollama_instance.clear_chat();
    }
}

impl SQMStruct{

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
