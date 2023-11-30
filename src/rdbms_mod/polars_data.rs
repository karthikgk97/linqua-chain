use std::collections::HashMap;
use polars::prelude::*;
use crate::rdbms_mod::base_rdbms::BaseRDBMSTrait;
use polars_sql::SQLContext;
use polars_lazy::frame::IntoLazy;

pub struct PolarsDataStruct{
    df: DataFrame
}

impl BaseRDBMSTrait for PolarsDataStruct{
    type RDBMSDataType = DataFrame;

    fn new(csv_file_path: &str) -> Self{
        let dataframe_from_csv = CsvReader::from_path(csv_file_path).unwrap().finish().unwrap();

        return PolarsDataStruct{
            df: dataframe_from_csv
        }
    }

    fn get_column_names(&self) -> Vec<HashMap<String, String>> {
        log::info!("Retrieving Column Names for df");
        let column_names_with_type: Vec<HashMap<String, String>> = self.df.get_column_names().iter().map(|col| {
            let mut tmp_hashmap = HashMap::new();
            let col_data_type = self.df.column(col).unwrap().dtype().to_string();
            tmp_hashmap.insert(col.to_string(), col_data_type);
            tmp_hashmap
        }).collect();
    
        return column_names_with_type;
    }

    fn get_distinct_options(&self, column_name: &str) -> Vec<String> {
        log::info!("Retrieving distinct options for column name {}", column_name);
        let unique_options = self.df.select([column_name.to_string()]).unwrap().unique(None, UniqueKeepStrategy::Any, None);

        return unique_options.unwrap()[column_name].iter().map(|col| col.to_string()).collect();
    }

    fn execute_sql_query(&self, sql_query_to_execute: &str) -> Self::RDBMSDataType{
        let mut ctx = SQLContext::new();
        ctx.register("df", self.df.clone().lazy());
        let sql_df = ctx.execute(sql_query_to_execute).unwrap().collect().unwrap();
        log::info!("SQL OP is {:?}", sql_df);

        return sql_df;
    }
}
