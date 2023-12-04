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


#[tokio::main]
async fn main() {
    Builder::new().filter_level(log::LevelFilter::Info).init();

    let pds = PolarsDataStruct::new("./auto_sales_test.csv");
    let pds_columns: HashMap<String, String> = pds.get_column_names();

    log::info!("Available columns is {:?}", pds_columns);

    let mut ccm = CCMStruct::new("http://localhost", Some("llama2"), true);

    ccm.set_temperature(0.7);
    ccm.set_max_output_length(200);

    let available_columns: Vec<String> = pds_columns.clone().into_keys().collect();

    log::info!("Available columns is {:?}", available_columns);

    let default_system_message = ccm.get_default_system_message(available_columns.clone());
    
    ccm.set_system_prompt(&default_system_message);

    let user_input_question = "Sale price for Customer Name Corrida";

    let chat_op = ccm.chat(user_input_question).await;
    let ccm_chat_op_list = ccm.string_to_list(chat_op);
    log::info!("CCM Chat list OP {:?}", ccm_chat_op_list);



    let cdom = CDOMStruct::new(None, None);
    // let _ = cdom.delete_collection("test_collection").await;
    // let _ = cdom.create_collection("test_collection").await;

    // let mut search_results_vec: Vec<String, HashMap<String, f64>> = Vec::new();
    let mut search_results_vec: Vec<HashMap<&String, Vec<String>>> = Vec::new();
    for col_name in ccm_chat_op_list.iter(){
        let filter = hashmap!{"column_name" => col_name.as_str()};
        let search_result_response = cdom.search_collection(
            "auto_sales",
            user_input_question,
            Some(filter),
            10
        ).await;

        // let tmp_list: Vec<_> = search_result_response.into_keys();

        // let tmp_store: Vec<_> = search_result_response.into_keys().map(|x| {
        //     return (col_name, x);
        // }).collect();
        // log::info!("Keys is {:?}", search_result_response.keys().cloned().collect());
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



    let mut sqm = SQMStruct::new("http://localhost", Some("llama2"), true);
    sqm.set_temperature(0.7);
    sqm.set_max_output_length(200);
    
    let table_name = "df";
    let table_description = "Contains information about Automotive Sales";

    let sqm_default_system_message = sqm.get_default_system_message(table_name, table_description, available_columns.clone());
    sqm.set_system_prompt(&sqm_default_system_message);


    // let columns_list_str = format!("[\"{}\"]", available_columns_list.join("\",\""));

    let sqm_user_input_question = format!("User Input Question: Total sales for Customer Name Corrida.
    These are the closest options found that might be benefical for generating the SQL Query: {:?}", search_results_vec);

    log::info!("SQM input quesiton is {}", sqm_user_input_question);

    let chat_op = sqm.chat(&sqm_user_input_question).await;

    let _ = pds.execute_sql_query(&chat_op);


    // 

    // 



    // pds.execute_sql_query("SELECT * FROM df");
    

    // let _ = qdb_client.create_collection("test_collection").await;
    // let documents = vec![
    //     "this is test document 1".to_string(),
    //     "this is test document 2".to_string(),
    //     "this is test document 3".to_string(),
    //     "this is test document 4".to_string(),
    //     "this is test document 5".to_string(),
    //     "this is test document 6".to_string()
    //     ];

    // let metadata_for_stuff: Vec<HashMap<String, String>>= vec![
    //     hashmap!{"document_name".to_string() => "test".to_string()},
    //     hashmap!{"document_name".to_string() => "test".to_string()},
    //     hashmap!{"document_name".to_string() => "test".to_string()},
    //     hashmap!{"document_name".to_string() => "dummy".to_string()},
    //     hashmap!{"document_name".to_string() => "dummy".to_string()},
    //     hashmap!{"document_name".to_string() => "dummy".to_string()}
    //     ];

    // let id_for_stuff: Vec<u64> = vec![1, 2, 3, 4, 5, 6];

    // let _ = qdb_client.add_stuff_to_collection("test_collection", documents, id_for_stuff, metadata_for_stuff).await;

    // let _ = qdb_client.search_collection("test_collection", "document 1 ", Some(hashmap!{"document_name" => "test"}), 5).await;

}