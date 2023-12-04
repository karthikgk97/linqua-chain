use env_logger::Builder;
use std::collections::HashMap;
use maplit::hashmap;

use linqua_chain::rdbms_mod::base_rdbms::BaseRDBMSTrait;
use linqua_chain::rdbms_mod::polars_data::PolarsDataStruct;
use linqua_chain::llm_mod::base_llm::BaseLLMTrait;
use linqua_chain::structured_data::ccm::CCMStruct;
use linqua_chain::structured_data::cdom::CDOMStruct;
use linqua_chain::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use linqua_chain::vectordb_mod::qdrant_db::QdrantDBStruct;


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

    let default_system_message = ccm.get_default_system_message(available_columns);
    
    ccm.set_system_prompt(&default_system_message);

    let chat_op = ccm.chat("price for customer Jack Hogan").await;
    let ccm_chat_op_list = ccm.string_to_list(chat_op);
    log::info!("CCM Chat list OP {:?}", ccm_chat_op_list);



    let cdom = CDOMStruct::new(None, None);
    let cdom_collection_name = "auto_sales";
    // deleting collection if any
    let _ = cdom.delete_collection(cdom_collection_name).await;
    let _ = cdom.create_collection(cdom_collection_name).await;

    
    for col_name in pds_columns.clone().into_keys(){
        let col_name_str: &str = &col_name; // Convert String to &str
        if ["ORDERNUMBER", "PRICEEACH", "QUANTITYORDERED", "MSRP", "DAYS_SINCE_LASTORDER", "ORDERLINENUMBER", "SALES"].contains(&col_name_str)
        {
            continue;
        }
        let pds_dis = pds.get_distinct_options(&col_name);
        log::info!("Num of distinct options for column name {} is {}", col_name, pds_dis.len());

        let id_for_stuff = QdrantDBStruct::create_ids(pds_dis.clone());
        let metadata_for_stuff = vec![hashmap!{"column_name".to_string() => col_name.to_string()}; pds_dis.len()];

        let _ = cdom.add_stuff_to_collection(cdom_collection_name, pds_dis, id_for_stuff, metadata_for_stuff).await;
    }


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