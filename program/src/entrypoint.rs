use solana_program::account_info::AccountInfo;
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::processor::Processor;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    Processor::process_instruction(program_id, accounts, instruction_data)
}