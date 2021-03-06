use super::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::{HttpClient, No};

/// A client for Cosmos stored procedure resources.
#[derive(Debug, Clone)]
pub struct StoredProcedureClient {
    collection_client: CollectionClient,
    stored_procedure_name: ReadonlyString,
}

impl StoredProcedureClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection_client: CollectionClient,
        stored_procedure_name: S,
    ) -> Self {
        Self {
            collection_client,
            stored_procedure_name: stored_procedure_name.into(),
        }
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    pub fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    pub fn create_stored_procedure(&self) -> requests::CreateStoredProcedureBuilder<'_, '_, No> {
        requests::CreateStoredProcedureBuilder::new(self)
    }

    pub fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, '_, No> {
        requests::ReplaceStoredProcedureBuilder::new(self)
    }

    pub fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_> {
        requests::ExecuteStoredProcedureBuilder::new(self)
    }

    pub fn delete_stored_procedure(&self) -> requests::DeleteStoredProcedureBuilder<'_, '_> {
        requests::DeleteStoredProcedureBuilder::new(self)
    }

    pub fn prepare_request(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
            ResourceType::StoredProcedures,
        )
    }

    pub fn prepare_request_with_stored_procedure_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.stored_procedure_name()
            ),
            method,
            ResourceType::StoredProcedures,
        )
    }
}
