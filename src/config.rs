use anyhow::Result;
use dotenv;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::H160,
};
use std::{str::FromStr, sync::Arc};

#[derive(Debug)]
pub struct Config {
    pub ws: Arc<Provider<Ws>>,
    pub http_domain: String,
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub bot_address: H160,
    pub fork_port: u16,
    pub fork_chain_id: u64,
}

impl Config {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().unwrap();
        let ws_domain = std::env::var("WSS").expect("Failed to get WSS");
        let ws_provider = Provider::<Ws>::connect(ws_domain).await?;
        let ws_provider = Arc::new(ws_provider);

        let http_domain = std::env::var("HTTP").expect("Failed to get HTTP");
        let http_provider = Provider::<Http>::try_from(&http_domain)?;

        let fork_port = std::env::var("FORK_PORT")
            .ok()
            .map(|port| port.parse::<u16>().unwrap())
            .unwrap_or_else(|| 8545);
        let fork_chain_id = std::env::var("FORK_CHAIN_ID")
            .ok()
            .map(|chain_id| chain_id.parse::<u64>().unwrap())
            .unwrap_or_else(|| 31337);

        let chain_id = ws_provider.get_chainid().await?;

        let private_key = std::env::var("PRIVATE_KEY").expect("Failed to get Private Key");
        let wallet = private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let middleware = SignerMiddleware::new(http_provider, wallet);

        // Add your mainnet address here
        let bot_address =
            H160::from_str(&String::from("0x4162a3316fb46e2c9e1cedf459c42f669dcabc3e"))?;

        Ok(Self {
            ws: ws_provider,
            http_domain,
            http: Arc::new(middleware),
            bot_address,
            fork_port,
            fork_chain_id,
        })
    }
}
