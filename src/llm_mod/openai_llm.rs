use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

pub struct OpenAILLMStruct{
    model_name: String,
    messages: Vec<HashMap<String, String>>,
    temperature: f32,
    top_p: f32,
    // top_k: u32,
    max_output_length: i32,
    track_history: bool
}

impl OpenAILLMStruct{
    pub fn new(model_name: Option<&str>) -> Self{
        log::info!("Initializing OpenAI LLM Struct");
        let default_model_name = "gpt-3.5-turbo";
        let default_track_history: bool = false;

        return OpenAILLMStruct{
            model_name: model_name.unwrap_or(default_model_name).to_string(),
            messages: Vec::new(),
            temperature: 0.7,
            top_p: 0.95,
            // top_k: 0.9,
            max_output_length: 256,
            track_history: default_track_history
        };
    }

    pub fn set_temperature(&mut self, new_temperature: f32){
        log::info!("Setting temperature as {}", new_temperature);
        self.temperature = new_temperature;
    }

    pub fn set_top_p(&mut self, new_top_p: f32){
        log::info!("Setting top_p as {}", new_top_p);
        self.top_p = new_top_p;
    }

    // pub fn set_top_k(&mut self, new_top_k: f32){
    //     log::info!("Setting top_p as {}", new_top_p);
    //     self.top_k = new_top_k;
    // }


    pub fn set_max_output_length(&mut self, new_max_output_length: i32){
        log::info!("Setting max output length as {}", new_max_output_length);
        self.max_output_length = new_max_output_length;
    }

    pub fn set_history_tracking(&mut self, track_history: bool){
        log::info!("Setting history tracking as {}", track_history);
        self.track_history = track_history;
    }

    pub fn set_custom_system_prompt(&mut self, system_prompt_message: &str){
        log::info!("Setting custom system prompt");

        if !self.messages.is_empty() {
            log::warn!("Messages are not empty. Clearing them to set the system message");
            self.messages.clear();
        }
        
        self.messages.push(HashMap::from(
            [
                ("role".to_string(), "system".to_string()),
                ("content".to_string(), system_prompt_message.to_string())
            ]
        ));
    }

    pub fn clear_chat(&mut self){
        log::info!("Clearing messages");
        self.messages.clear();
    }

    pub async fn chat(&mut self, user_prompt: &str) -> String{

        // removing previous user messages
        if !self.track_history && self.messages.last().unwrap()["role"] == "user" {
            log::info!("Need to delete previous message as it is of user type and history tracking is false");
            self.messages.pop();
        }


        self.messages.push(HashMap::from(
            [
                ("role".to_string(), "user".to_string()),
                ("content".to_string(), user_prompt.to_string())
            ]
        ));

        log::info!("Making a chat call with message {:?}", self.messages);

        let payload = json!({
            "model": self.model_name,
            "messages": self.messages,
            "temperature": self.temperature
        });

        let client = reqwest::Client::new();

        let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in environment");
        let url = "https://api.openai.com/v1/chat/completions";
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", openai_api_key).parse().unwrap());

        let response = client.post(url)
        .headers(headers)
        .body(serde_json::to_string(&payload).expect("Failed to serialize JSON"))
        .send()
        .await;

        match response {
            Ok(response) => {
                if response.status().is_success(){
                    let response_body = response.text().await;

                    match response_body{
                        Ok(response_body) => {
                            let json_body: Value = serde_json::from_str(&response_body).expect("Failed to parse JSON");

                            let input_token_count = &json_body["usage"]["prompt_tokens"];
                            let output_token_count = &json_body["usage"]["completion_tokens"];
                            log::info!("Input Token Usage is {}", input_token_count);
                            log::info!("Output Token Usage is {}", output_token_count);

                            let llm_response = &json_body["choices"][0]["message"]["content"];
                            log::info!("LLM response is {}", llm_response);
                            
                            if self.track_history{
                                let modified_response = llm_response.as_str().unwrap().trim_matches('"').to_string();
                                self.messages.push(HashMap::from(
                                    [
                                        ("role".to_string(), "assistant".to_string()),
                                        ("content".to_string(), modified_response)
                                    ]
                                ))
                            }

                            return llm_response.to_string();
                        }
                        Err(err) => {
                            log::error!("Response Success. But Errored on Response body with error {}", err);
                            return "error_in_api_call".to_string();
                        }
                    }
                }
                else{
                    log::error!("Response Failed. Errored due to error {}", response.status());
                    return "error_in_api_call".to_string();
                }
            }
            Err(err) => {
                log::error!("Errored on making OpenAI API call with the error message {}", err);
                return "error_in_api_call".to_string();
            }
        }
    }

}