use std::collections::HashMap;

pub trait BaseRDBMSTrait {
    type RDBMSDataType;
    // fn for initiating the object
    fn new(csv_file_path: &str) -> Self;

    // fn for getting column names along with its type
    fn get_column_names(&self) -> HashMap<String, String>;

    // fn for getting distinct options for given column name
    fn get_distinct_options(&self, column_name: &str) -> Vec<String>;

    fn execute_sql_query(&self, sql_query_to_execute: &str) -> Self::RDBMSDataType;
}