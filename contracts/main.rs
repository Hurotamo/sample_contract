use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

mod contracts {
    pub mod minting;
    pub mod burning;
    pub mod vesting;
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction data to determine which function to call
    match instruction_data[0] {
        0 => contracts::minting::mint_tokens(accounts, 1000), // Example mint amount
        1 => contracts::burning::burn_tokens(accounts, 500),  // Example burn amount
        2 => contracts::vesting::vest_tokens(accounts, 1622505600, 30 * 24 * 60 * 60, 365 * 24 * 60 * 60), // Example vesting parameters
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
    }
}
