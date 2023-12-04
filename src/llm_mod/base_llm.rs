use async_trait::async_trait;
// use serde::{Deserialize, Serialize};

#[async_trait]
pub trait BaseLLMTrait {
    fn new(model_endpoint: &str, model_name: Option<&str>, track_history: bool) -> Self;

    fn set_system_prompt(&mut self, system_prompt_message: &str);
    fn set_temperature(&mut self, new_temperature: f32);
    fn set_top_p(&mut self, new_top_p: f32);
    fn set_top_k(&mut self, new_top_k: u32);
    fn set_max_output_length(&mut self, new_max_output_length: i32);
    async fn chat(&mut self, user_prompt: &str) -> String;
    fn clear_chat(&mut self); 
}