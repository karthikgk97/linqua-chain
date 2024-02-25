// file containing the CORE LLM Config


#[derive(Debug, Clone, PartialEq)]
pub enum OpenAILLMModels{
    Gpt35_4k,
    Gpt35_16k,
    Gpt4_8k,
    Gpt4_32k,
    Gpt4_128k
}

#[derive(Debug, Clone, PartialEq)]
pub enum LLMModelName {
    OpenAI(OpenAILLMModels)
}

#[derive(Debug, Clone, PartialEq)]
pub struct LLMConfig {
    pub model_name: LLMModelName,
    pub temperature: f32,
    pub top_p: f32,
    pub max_output_tokens: u32
}


#[derive(Debug, Clone, PartialEq)]
pub struct LLMOutputResponse {
    pub output_response: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32
}
