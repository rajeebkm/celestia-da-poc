pub mod traits;
pub mod celestia;
pub mod eigenlayer;

use crate::adapters::traits::AdapterFunctions;
use crate::adapters::celestia::CelestiaAdapter;
use crate::adapters::eigenlayer::EigenLayerAdapter;
use crate::config::AdapterConfiguration;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum AdapterTypes {
    Celestia,
    EigenLayer,
}

pub async fn get_adapter(adapter_type: AdapterTypes, config: &AdapterConfiguration) -> Arc<Mutex<dyn AdapterFunctions + Send + Sync>> {
    match adapter_type {
        AdapterTypes::Celestia => {
            let adapter = CelestiaAdapter::new(&config.rpc_node, &config.auth_token).await;
            Arc::new(Mutex::new(adapter))
        }
        AdapterTypes::EigenLayer => {
            let adapter = EigenLayerAdapter::new(&config.rpc_node, &config.auth_token).await;
            Arc::new(Mutex::new(adapter))
        }
    }
}
