// file containing openAI code

use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::Instant;

use crate::core::llm_config::{LLMConfig, LLMModelName, LLMOutputResponse, OpenAILLMModels};

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAILLMConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum OpenAILLMError {
    InvalidModelType,
    LLMChatError,
}

impl OpenAILLMConfig {
    pub fn get_llm_config(
        llm_model_name: OpenAILLMModels,
        temperature: f32,
        top_p: f32,
        max_output_tokens: u32,
    ) -> LLMConfig {
        log::info!("Retrieving LLM Config");

        LLMConfig {
            model_name: LLMModelName::OpenAI(llm_model_name),
            temperature,
            top_p,
            max_output_tokens,
        }
    }

    pub fn format_message(role: String, content: String) -> HashMap<String, String> {
        HashMap::from([
            (String::from("role"), role),
            (String::from("content"), content),
        ])
    }

    pub async fn chat(
        llm_config: LLMConfig,
        chat_history: &mut Vec<HashMap<String, String>>,
        user_query: String,
    ) -> Result<LLMOutputResponse, OpenAILLMError> {
        let total_function_time = std::time::Instant::now();

        log::info!("Making Chat Request for User Query {}", user_query);

        let model_name = match llm_config.model_name {
            LLMModelName::OpenAI(openai_model) => match openai_model {
                OpenAILLMModels::Gpt35_4k => String::from("gpt-3.5-turbo"),
                OpenAILLMModels::Gpt35_16k => String::from("gpt-35-turbo-16k"),
                OpenAILLMModels::Gpt4_8k => String::from("gpt-4"),
                OpenAILLMModels::Gpt4_32k => String::from("gpt-4-32k"),
                OpenAILLMModels::Gpt4_128k => String::from("gpt-4-0613"),
            },
        };

        chat_history.push(Self::format_message(
            String::from("user"),
            user_query.clone(),
        ));
        let openai_api_key =
            std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in environment");
        let url = String::from("https://api.openai.com/v1/chat/completions");

        // adding the API KEY in the headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", openai_api_key).parse().unwrap(),
        );

        // initialize the request client
        let client = reqwest::Client::new();

        //configuring the payload
        let payload = json!({
            "model": model_name,
            "messages": chat_history,
            "temperature" : llm_config.temperature,
            "top_p": llm_config.top_p,
            "max_tokens": llm_config.max_output_tokens
        });

        log::info!("Making request for query {}", user_query);
        let request_start_time = std::time::Instant::now();
        let response = client
            .post(url)
            .headers(headers)
            .body(serde_json::to_string(&payload).expect("Failed to serialize JSON"))
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let response_body = response.text().await;

                    match response_body {
                        Ok(response_body) => {
                            let json_body: Value =
                                serde_json::from_str(&response_body).expect("Failed to parse JSON");

                            let input_token_count = &json_body["usage"]["prompt_tokens"];
                            let output_token_count = &json_body["usage"]["completion_tokens"];
                            log::info!("Input Token Usage is {}", input_token_count);
                            log::info!("Output Token Usage is {}", output_token_count);

                            let llm_response = &json_body["choices"][0]["message"]["content"];
                            log::info!("LLM response is {}", llm_response);
                            log::info!(
                                "Elapsed time for making Request {:.2?}",
                                request_start_time.elapsed()
                            );
                            log::info!(
                                "Total Time taken for making chat call: {:.2?}",
                                total_function_time.elapsed()
                            );
                            Ok(LLMOutputResponse {
                                output_response: llm_response.to_string().trim_matches('"').to_string(),
                                input_tokens: 1,
                                output_tokens: 1,
                                total_tokens: 1,
                            })
                        }
                        Err(err) => {
                            log::error!(
                                "Response Success. But Errored on Response body with error {}",
                                err
                            );
                            Err(OpenAILLMError::LLMChatError)
                        }
                    }
                } else {
                    log::error!(
                        "Response Failed with code {:?}",
                        response.error_for_status()
                    );
                    Err(OpenAILLMError::LLMChatError)
                }
            }
            Err(err) => {
                log::error!("Errored on making OpenAI API call with error msg {}", err);
                Err(OpenAILLMError::LLMChatError)
            }
        }
    }
}
