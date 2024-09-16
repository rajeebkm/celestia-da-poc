use async_trait::async_trait;

#[async_trait]
pub trait AdapterFunctions: Send + Sync {
    async fn push_data(&self, namespace_id: &[u8], data: &[u8]) -> Option<u64>;
    async fn pull_data(&self, height: u64, namespace_id: &[u8]) -> Result<Vec<u8>, String>;
}

