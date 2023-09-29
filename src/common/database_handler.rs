use std::fmt::Debug;
use mongodb::bson::{doc, from_document, to_document};
use mongodb::{Client, Collection, Database};
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::results::{DeleteResult, InsertOneResult};
use serde::{Serialize, Deserialize};
use crate::common::error::{DatabaseError, Error};
use crate::common::validation::Validateable;

pub struct DatabaseHandler {
    client: Client,
    db: Option<Database>,
    collection: Option<Collection<Document>>
}

impl DatabaseHandler {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let mut client_options = ClientOptions::parse(uri).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        Ok(DatabaseHandler { client, db: None, collection: None })
    }

    pub async fn connect_database(mut self, db_name: &str) -> Self {
        let db = self.client.database(db_name);

        self.db = Some(db);

        self
    }

    pub fn connect_collection(mut self, collection_name: &str) -> Result<DatabaseHandler, Error> {
        let collection = self.db.as_ref().ok_or("Database is not initialised").map_err(|e| {Error::DatabaseError(DatabaseError::Generic(e.to_string()))})?.collection::<Document>(collection_name);

        self.collection = Some(collection);

        Ok(self)
    }

    pub async fn add<T: Serialize + Validateable<T> + Debug>(&self, item: T) -> Result<InsertOneResult, Error> {
        if item.validate_add(&self) == true {
            return Err(Error::DatabaseError(DatabaseError::ExistingItem(format!("Item already exists: {:?}", item))));
        }

        let collection = self.collection.as_ref().ok_or_else(|| {
            Error::DatabaseError(DatabaseError::Generic("Collection is not initialised".to_string()))
        })?;

        Ok(collection.insert_one(to_document(&item).unwrap(), None).await?)
    }

    pub async fn remove<T: Serialize>(&self, name: &str) -> Result<DeleteResult, Error> {
        let filter = doc! { "name": name };

        let collection = self.collection.as_ref().ok_or_else(|| {
            Error::DatabaseError(DatabaseError::Generic("Collection is not initialised".to_string()))
        })?;

        Ok(collection.delete_one(filter, None).await?)
    }

    pub async fn get<T: Serialize +  for<'a> Deserialize<'a>>(&self, name: &str) -> Result<Option<T>, Error> {
        let filter = doc! { "name": name };

        let collection = self.collection.as_ref().ok_or_else(|| {
            Error::DatabaseError(DatabaseError::Generic("Collection is not initialised".to_string()))
        })?;

        let doc = collection.find_one(filter, None).await?;

        return match doc {
            None => Ok(None),
            Some(doc) => {
                let doc = from_document::<T>(doc).unwrap();

                Ok(Some(doc))
            }
        }
    }
}