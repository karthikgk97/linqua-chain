use maplit::hashmap;
use linqua_chain::vectordb_mod::base_vectordb::BaseVectorDBTrait;
use linqua_chain::vectordb_mod::qdrant_db::{QdrantDBStruct};
use async_trait::async_trait;

pub struct CDOMStruct{
    qdrant_instance: QdrantDBStruct
}

impl CDOMStruct{
    pub fn new(){
        
    }
}

let qdb_client = QdrantDBStruct::new(None, None);