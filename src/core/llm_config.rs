// file containing the CORE LLM Config

#[derive(Clone, Debug, PartialEq)]
enum LLMModelType {
    OpenAI,
    Gemini
}


#[derive(Clone, Debug, PartialEq)]
struct LLMConfig {
    llm_model_type: LLMModelType,
    llm_model_name: String,
    llm_model_temperature: f32,
    llm_model_top_p: f32,
    llm_model_max_output_tokens: u32
}


#[derive(Debug, Clone, PartialEq)]
struct LLMOutputResponse {
    output_response: String,
    input_tokens: u32,
    output_tokens: u32,
    total_tokens: u32
}
