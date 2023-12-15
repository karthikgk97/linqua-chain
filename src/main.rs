use env_logger::Builder;
use std::collections::HashMap;
use maplit::hashmap;
use linqua_chain::rdbms_mod::base_rdbms::BaseRDBMSTrait;
use linqua_chain::rdbms_mod::polars_data::PolarsDataStruct;
use linqua_chain::llm_mod::base_llm::BaseLLMTrait;
use linqua_chain::structured_data::ccm::CCMStruct;
use linqua_chain::structured_data::cdom::CDOMStruct;
use linqua_chain::structured_data::sqm::SQMStruct;
use linqua_chain::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let pds = PolarsDataStruct::new("./auto_sales_test.csv");
    let pds_columns: HashMap<String, String> = pds.get_column_names();

    log::info!("Available columns is {:?}", pds_columns);

    // let mut ccm = CCMStruct::new("http://localhost", Some("openhermes2.5-mistral:latest"), true);
    let mut ccm = CCMStruct::new("http://localhost", Some("llama:latest"), true);
    

    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let res = ollama.list_local_models().await.unwrap();

    println!("Models available {:?}", res);

    ccm.set_temperature(0.7);
    ccm.set_max_output_length(200);

    let available_columns: Vec<String> = pds_columns.clone().into_keys().collect();

    log::info!("Available columns is {:?}", available_columns);

    let default_system_message = ccm.get_default_system_message(available_columns.clone());
    
    ccm.set_system_prompt(&default_system_message);

    let user_input_question = "Sale price for ADDRESSLINE1: '815 pacific highway' Customer Name: Corrida";

    let chat_op = ccm.chat(user_input_question).await;
    let ccm_chat_op_list = ccm.string_to_list(chat_op);
    log::info!("CCM Chat list OP {:?}", ccm_chat_op_list);

    let cdom = CDOMStruct::new(None, None);
    let mut search_results_vec: Vec<HashMap<&String, Vec<String>>> = Vec::new();
    for col_name in ccm_chat_op_list.iter(){
        let filter = hashmap!{"column_name" => col_name.as_str()};
        let search_result_response = cdom.search_collection(
            "auto_sales",
            user_input_question,
            Some(filter),
            10
        ).await;

       
        let tmp_store: HashMap<&String, Vec<String>> = hashmap!(col_name => search_result_response.keys().cloned().collect::<Vec<String>>());
        log::info!("Tmp store is {:?}", tmp_store);
        search_results_vec.push(tmp_store);

        log::info!("Search result vec is {:?}", search_results_vec);

        // for key_val in tmp_list.iter(){
        //     // let mut modified_key_val = key_val.trim_matches('"');
        //     // let unquoted = &modified_key_val.replace(r#"\"#, r#""#);
        //     log::info!("Modified key val is {}", key_val);
        // }
        // log::info!("Keys is {:?}", tmp_list);
        // let tmp_hashmap = hashmap!(col_name => tmp_list);
        // search_results_vec.push(tmp_hashmap);
    }

    let mut sqm = SQMStruct::new("http://localhost", Some("codellama:latest"), true);
    sqm.set_temperature(0.7);
    sqm.set_max_output_length(200);
    
    let table_name = "df";
    let table_description = "Contains information about Automotive Sales";

    let sqm_default_system_message = sqm.get_default_system_message(table_name, table_description, available_columns.clone());
    sqm.set_system_prompt(&sqm_default_system_message);

    let sqm_user_input_question = format!("User Input Question: {}.
    These are the closest options found that might be benefical for generating the SQL Query: {:?}", &user_input_question, search_results_vec);

    log::info!("SQM input quesiton is {}", sqm_user_input_question);

    let chat_op = sqm.chat(&sqm_user_input_question).await;

    println!("Chat OP is {}", chat_op);

    let _ = pds.execute_sql_query(&chat_op);

    let user_input_question = "What is the Sales for Dealsize: small, Productline: Planes for 2021?";

    let chat_op = ccm.chat(user_input_question).await;
    let ccm_chat_op_list = ccm.string_to_list(chat_op);
    log::info!("CCM Chat list OP {:?}", ccm_chat_op_list);

    let cdom = CDOMStruct::new(None, None);
    let mut search_results_vec: Vec<HashMap<&String, Vec<String>>> = Vec::new();
    for col_name in ccm_chat_op_list.iter(){
        let filter = hashmap!{"column_name" => col_name.as_str()};
        let search_result_response = cdom.search_collection(
            "auto_sales",
            user_input_question,
            Some(filter),
            10
        ).await;

        let tmp_store: HashMap<&String, Vec<String>> = hashmap!(col_name => search_result_response.keys().cloned().collect::<Vec<String>>());
        log::info!("Tmp store is {:?}", tmp_store);
        search_results_vec.push(tmp_store);

        log::info!("Search result vec is {:?}", search_results_vec);

        // for key_val in tmp_list.iter(){
        //     // let mut modified_key_val = key_val.trim_matches('"');
        //     // let unquoted = &modified_key_val.replace(r#"\"#, r#""#);
        //     log::info!("Modified key val is {}", key_val);
        // }
        // log::info!("Keys is {:?}", tmp_list);
        // let tmp_hashmap = hashmap!(col_name => tmp_list);
        // search_results_vec.push(tmp_hashmap);
    }

    let mut sqm = SQMStruct::new("http://localhost", Some("codellama:latest"), true);
    sqm.set_temperature(0.7);
    sqm.set_max_output_length(200);
    
    let table_name = "df";
    let table_description = "Contains information about Automotive Sales";

    let sqm_default_system_message = sqm.get_default_system_message(table_name, table_description, available_columns.clone());
    println!("Default system message is {}", &sqm_default_system_message);
    sqm.set_system_prompt(&sqm_default_system_message);

    let sqm_user_input_question = format!("User Input Question: {}.
    These are the closest options found that might be benefical for generating the SQL Query: {:?}", &user_input_question, search_results_vec);

    log::info!("SQM input quesiton is {}", sqm_user_input_question);

    let chat_op = sqm.chat(&sqm_user_input_question).await;

    println!("Chat OP is {}", chat_op);

    let _ = pds.execute_sql_query(&chat_op);



}