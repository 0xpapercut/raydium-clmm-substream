use base64;
use bs58;

use serde::{Deserialize, Serialize};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

mod pb;

#[derive(Serialize, Deserialize)]
pub struct SwapEvent {
    pub pool_state: [u8; 32],
    pub sender: [u8; 32],
    pub token_account_0: [u8; 32],
    pub token_account_1: [u8; 32],
    pub amount_0: u64,
    pub transfer_fee_0: u64,
    pub amount_1: u64,
    pub transfer_fee_1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}

#[substreams::handlers::map]
fn raydium_clmm_swap_events(block: Block) -> Result<pb::raydium_clmm::SwapEvents, Error> {
    let mut events: Vec<pb::raydium_clmm::SwapEvent> = Vec::new();
    for transaction in block.transactions {
        let meta = transaction.meta.as_ref().unwrap();
        if let Some(_) = meta.err {
            continue;
        }
        let log_messages = &meta.log_messages;
        for (i, log_message) in log_messages.iter().enumerate() {
            if log_message.starts_with("Program Data: ") {
                if let Some(data) = log_message.strip_prefix("Program Data: ") {
                    if let Some(Some(_)) = log_messages.get(i).map(|s| {
                        s.strip_prefix("Program CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK")
                    }) {
                        let bytes = base64::decode(data).unwrap();
                        if let Ok(event) = bincode::deserialize::<SwapEvent>(&bytes) {
                            events.push(pb::raydium_clmm::SwapEvent {
                                pool_state: bs58::encode(event.pool_state).into_string(),
                                sender: bs58::encode(event.sender).into_string(),
                                token_account_0: bs58::encode(event.token_account_0).into_string(),
                                token_account_1: bs58::encode(event.token_account_1).into_string(),
                                amount_0: event.amount_0,
                                transfer_fee_0: event.transfer_fee_0,
                                amount_1: event.amount_1,
                                transfer_fee_1: event.transfer_fee_1,
                                zero_for_one: event.zero_for_one,
                                sqrt_price_x64: event.sqrt_price_x64 as u64,
                                liquidity: event.liquidity as u64,
                                tick: event.tick,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(pb::raydium_clmm::SwapEvents { events })
}
