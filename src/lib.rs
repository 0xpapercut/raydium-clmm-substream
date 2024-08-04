use bs58;

use pb::raydium_clmm::SwapEvents;
use serde::{Deserialize, Serialize};
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};
use substreams_solana_utils::{
    get_structured_instructions,
    TransactionContext,
    StructuredInstructions,
};

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

static RAYDIUM_CLMM_PROGRAM_ID: &'static str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

#[substreams::handlers::map]
fn raydium_clmm_swap_events(block: Block) -> Result<pb::raydium_clmm::SwapEvents, Error> {
    for transaction in &block.transactions {
        parse_transaction(transaction);
    }
    Ok(SwapEvents { events: Vec::new() })
}

fn parse_transaction(confirmed_transaction: &ConfirmedTransaction) {
    if let Some(_) = confirmed_transaction.meta.as_ref().unwrap().err {
        return;
    }

    let instructions = get_structured_instructions(confirmed_transaction).unwrap();
    let context = TransactionContext::construct(confirmed_transaction);

    for instruction in instructions.flattened() {
        let program_id = bs58::encode(context.get_account_from_index(instruction.program_id_index() as usize)).into_string();
        if program_id == RAYDIUM_CLMM_PROGRAM_ID {
            substreams::log::println(format!("{:#?}", instruction.logs));
        }
    }
}
