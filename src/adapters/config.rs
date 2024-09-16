use crate::adapters::traits::AdapterFunctions;
use crate::adapters::celestia::CelestiaAdapter;
use crate::adapters::eigenlayer::EigenLayerAdapter;
use std::sync::Arc;
use tokio::sync::Mutex;
use dotenv::dotenv;

pub enum AdapterTypes {
    Celestia,
    EigenLayer,
}

pub struct AdapterConfiguration {
    pub rpc_node: String,
    pub auth_token: String,
}

pub async fn fetch_adapter(adapter_type: AdapterTypes) -> (Arc<Mutex<dyn AdapterFunctions + Send + Sync>>, AdapterConfiguration) {
    dotenv().ok(); // Load environment variables from .env
    let (adapter, _adapter_config) = match adapter_type {
        AdapterTypes::Celestia => {
            let config = AdapterConfiguration {
                rpc_node: std::env::var("RPC_NODE_CELESTIA").expect("RPC_NODE_CELESTIA not set"),
                auth_token: std::env::var("AUTH_TOKEN_CELESTIA_ARABICA").expect("AUTH_TOKEN_CELESTIA_ARABICA not set"),
            };
            (get_adapter(AdapterTypes::Celestia, &config).await, config) // Pass config by reference
        },
        AdapterTypes::EigenLayer => {
            let config = AdapterConfiguration {
                rpc_node: std::env::var("RPC_NODE_EIGENLAYER").expect("RPC_NODE_EIGENLAYER not set"),
                auth_token: std::env::var("AUTH_TOKEN_EIGENLAYER").expect("AUTH_TOKEN_EIGENLAYER not set"),
            };
            (get_adapter(AdapterTypes::EigenLayer, &config).await, config) // Pass config by reference
        },
    };
    (adapter, _adapter_config)
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


