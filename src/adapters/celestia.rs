use async_trait::async_trait;
use celestia_rpc::{BlobClient, Client};
use celestia_types::{Blob, TxConfig, nmt::Namespace};
use crate::adapters::traits::AdapterFunctions;

pub struct CelestiaAdapter {
    client: Client,
}

impl CelestiaAdapter {
    pub async fn new(url: &str, token: &str) -> Self {
        let client = Client::new(url, Some(token))
            .await
            .expect("Failed to create RPC client");
        CelestiaAdapter { client }
    }
}

#[async_trait]
impl AdapterFunctions for CelestiaAdapter {
    async fn push_data(&self, namespace_id: &[u8], data: &[u8]) -> Option<u64> {
        let namespace = Namespace::new_v0(namespace_id).expect("Invalid namespace");
        let blob = Blob::new(namespace, data.to_vec()).expect("Blob creation failed");

        let height = self.client
            .blob_submit(&[blob.clone()], TxConfig::default())
            .await
            .ok()?;
        
        Some(height)
    }

    async fn pull_data(&self, height: u64, namespace_id: &[u8]) -> Result<Vec<u8>, String> {
        let namespace = Namespace::new_v0(namespace_id).map_err(|_| "Invalid namespace".to_string())?;
        let retrieved_blobs = self.client.blob_get_all(height, &[namespace]).await.expect("Failed getting blobs").unwrap();
        assert_eq!(retrieved_blobs.len(), 1);
        println!("Retrieved blob: {:?}", String::from_utf8(retrieved_blobs[0].data.to_vec()));
        let blob_data = retrieved_blobs[0].data.to_vec();
        Ok(blob_data)
    
        // // Attempt to retrieve blobs, returning an error if it fails
        // let retrieved_blobs = self.client
        //     .blob_get_all(height, &[namespace])
        //     .await
        //     .map_err(|_| "Failed getting blobs".to_string())?; 

        // Convert blob data to a UTF-8 string and return it
        // let blob_data = retrieved_blobs[0].data.to_vec();
        // println!("Retrieved blob: {:?}", String::from_utf8(blob_data.clone()).unwrap_or_else(|_| "Invalid UTF-8".to_string()));
        // Ok(blob_data)  // Return the blob data as a Vec<u8>
    }

    // async fn pull(&self, height: u64, namespace_id: &[u8]) -> Option<Vec<u8>> {
    //     let namespace = Namespace::new_v0(namespace_id).expect("Invalid namespace");
    //     let retrieved_blobs = self.client
    //         .blob_get_all(height, &[namespace])
    //         .await
    //         .ok()?;

    //     Some(retrieved_blobs.into_iter().flat_map(|blob| blob.data).collect())
    // }
}


