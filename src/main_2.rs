use env_logger::Builder;
use linqua_chain::llm_mod::openai_llm::OpenAILLMStruct;

#[tokio::main]
pub async fn main(){
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let mut openai_llm = OpenAILLMStruct::new(None);

    openai_llm.set_temperature(0.5);
    openai_llm.set_history_tracking(true);

    openai_llm.set_custom_system_prompt("You are a good assistant");

    openai_llm.chat("Multiply 5 and 10").await;

    openai_llm.chat("How about 2 and 7").await;
}